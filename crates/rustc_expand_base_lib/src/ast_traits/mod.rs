//! This module defines a set of traits that break down the functionality
//! previously monolithic in `InvocationCollectorNode`. This promotes better
//! modularity, reusability, and adherence to the single-responsibility principle.

use rustc_ast::{self as ast, AttrVec, HasAttrs, HasNodeId, NodeId};
use rustc_span::{Ident, Span, Symbol};
use smallvec::SmallVec;
use crate::prelude::{Annotatable, AstFragment, AstFragmentKind, AddSemicolon, StripUnconfigured};
use rustc_session::lint::Lint;
use rustc_errors::Diagnostic;

// Trait for macro invocation related methods
pub trait MacroInvocationNode: HasAttrs + HasNodeId + Sized {
    type ItemKind; // Associated type for ast::ItemKind
    fn is_mac_call(&self) -> bool;
    fn take_mac_call(self) -> (ast::MacCall, AttrVec, AddSemicolon);
    fn delegation(&self) -> Option<(&ast::MacCall, &ast::AssocItem)>;
    fn delegation_item_kind(_deleg: Box<ast::Delegation>) -> Self::ItemKind;
    fn from_item(_item: ast::Item<Self::ItemKind>) -> Self;
}

// Trait for AST fragment conversion methods
pub trait AstFragmentConverter: Sized {
    type VisitOutputTy;
    type FlatMapOutputTy: Default;
    const KIND: AstFragmentKind; // Associated const for AstFragmentKind

    fn to_annotatable(self) -> Annotatable;
    fn fragment_to_visit_output(fragment: AstFragment) -> Self::VisitOutputTy;
    fn fragment_to_flat_map_output(fragment: AstFragment) -> Self::FlatMapOutputTy;
}

// Trait for walking/traversal methods
pub trait WalkableAstNode: Sized {
    type FlatMapOutputTy;
    fn walk<D>(&mut self, _collector: &mut D);
    fn walk_flat_map<D>(self, _collector: &mut D) -> Self::FlatMapOutputTy;
    fn wrap_flat_map_node_walk_flat_map<D>(
        node: Self,
        _collector: &mut D,
        walk_flat_map: impl FnOnce(Self, &mut D) -> Self::FlatMapOutputTy,
    ) -> Result<Self::FlatMapOutputTy, Self>;
}

// Trait for handling #[cfg(FALSE)] expansion logic
pub trait CfgFalseExpandable: Sized { // Removed + HasAttrs
    fn expand_cfg_false<D: CfgFalseReporterContext>(&mut self, collector: &mut D, pos: usize, span: Span);
}

// Trait for reporting declared identifiers
pub trait HasDeclaredIdents: Sized {
    fn declared_idents(&self) -> SmallVec<Ident, 1>;
}

// Trait for flattening outputs
pub trait FlattensOutputs: Sized {
    type FlatMapOutputTy;
    fn flatten_outputs(_outputs: impl Iterator<Item = Self::FlatMapOutputTy>) -> Self::FlatMapOutputTy;
}

// Trait for attribute collection hooks
pub trait AttributeHooks: Sized {
    fn pre_flat_map_node_collect_attr<F>(_cfg: &F, _attr: &ast::Attribute);
    fn post_flat_map_node_collect_bang(_output: &mut Self, _add_semicolon: AddSemicolon);
}

// Trait for abstracting the context needed for cfg_false reporting (CollectorContext)
pub trait CfgFalseReporterContext {
    fn buffer_lint_unused_attribute<'a>(
        &mut self,
        lint: Lint,
        span: Span,
        lint_node_id: NodeId,
        diag: impl Diagnostic<'a> + 'a,
    );
    fn get_unused_attribute_lint(&self, is_cfg: bool) -> Lint;

    fn get_lint_node_id(&self) -> NodeId; // Added this to pass lint_node_id directly
    fn get_strip_unconfigured(&self) -> StripUnconfigured;
}