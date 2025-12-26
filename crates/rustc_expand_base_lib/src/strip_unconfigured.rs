use rustc_ast::token::{Delimiter, Token, TokenKind};
use rustc_ast::tokenstream::{AttrTokenStream, AttrTokenTree, LazyAttrTokenStream, Spacing, TokenTree};
use rustc_ast::{self as ast, AttrKind, AttrStyle, Attribute, HasAttrs, HasTokens, MetaItem, MetaItemInner, NodeId, NormalAttr};
use crate::{configure, is_cfg};
use rustc_session::Session;
use rustc_feature::Features;
use rustc_span::symbol::sym;
use rustc_span::Span;

use std::iter;

/// A folder for `#[cfg]`-based conditional compilation.
///
/// This is a "folder" in the sense of `rustc_ast::visit::Folder`.
///
/// `strip_unconfigured_items` is the main entry point.
pub struct StripUnconfigured<'a> {
    pub sess: &'a rustc_session::Session,
    pub features: Option<&'a rustc_feature::Features>,
    /// If `true`, `#[cfg]` attributes are not stripped.
    pub keep_mode: KeepCfg,
    // Add missing fields
    pub config_tokens: bool,
    pub lint_node_id: NodeId,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum KeepCfg {
    No,
    Yes,
    IfChanged,
}

impl StripUnconfigured<'_> {
    /// Returns `true` if the attributes and expression are to be stripped.
    #[must_use]
    pub fn strip(
        &self,
        attrs: &[Attribute],
    ) -> bool {
        let mut strip = false;
        for attr in attrs {
            if !is_cfg(attr) {
                continue;
            }
            if let Some(meta_item) = attr.meta() {
                if !self.configure(&meta_item) {
                    strip = true;
                    break;
                }
            }
        }
        strip
    }

    /// `true` if the MetaItem is true
    #[must_use]
    fn configure(&self, meta_item: &MetaItem) -> bool {
        let MetaItem { path, kind, span, .. } = meta_item;
        configure(self.sess, self.features, path, kind, *span)
    }

    /// Strips any attributes that should not appear in the program based on the
    /// configuration of the session.
    pub fn process_cfg_attrs(&self, attrs: &mut ast::AttrVec) -> bool {
        let strip = self.strip(attrs);
        if self.keep_mode == KeepCfg::No {
            attrs.retain(|attr| !is_cfg(attr));
        }
        strip
    }
}
