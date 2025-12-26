use rustc_ast::{self as ast, Attribute};
use rustc_errors::ErrorGuaranteed;
use rustc_session::Session;
use rustc_span::{Ident, Span, Symbol};
use rustc_span::edition::Edition;
use std::sync::Arc;
use thin_vec::ThinVec;
use rustc_hir::def_id::DefId;
use std::any::Any;
use crate::resolver_traits::OpaqueDeriveResolution;

// MacroKindTrait represents the kind of macro (Bang, Attr, Derive, etc.)
// This corresponds to SyntaxExtensionKind from rustc_expand::base.
pub trait MacroKindTrait<DRT: OpaqueDeriveResolution + 'static>: Send + Sync + 'static {
    fn get_name(&self) -> String; // Placeholder for now
}

// SyntaxExtensionTrait defines the interface for a macro expander.
// This trait will be implemented by rustc_expand::base::SyntaxExtension.
pub trait SyntaxExtensionTrait<DRT: OpaqueDeriveResolution + 'static>: Send + Sync + 'static + Sized {
    // Constructor-like method. This will be called from compile_declarative_macro.
    fn new_macro_extension(
        sess: &Session,
        kind: Arc<dyn MacroKindTrait<DRT>>, // Use our MacroKindTrait
        span: Span,
        allow_internal_attrs: Vec<Attribute>,
        edition: Edition,
        name: Symbol,
        attrs: &[Attribute],
        is_local: bool,
    ) -> Self;

    // Helper methods for dummy extensions
    fn dummy_bang_extension(edition: Edition) -> Self;
    fn dummy_derive_extension(edition: Edition) -> Self;
    fn non_macro_attr_extension(edition: Edition) -> Self;
    fn glob_delegation_extension(def_id: DefId, impl_def_id: Option<DefId>, edition: Edition) -> Self;

    // Method to get the kind of the macro (e.g., Bang, Attr, Derive)
    fn get_macro_kind(&self) -> &dyn MacroKindTrait<DRT>;

    // Methods to downcast to specific expander types (returning &dyn Any for type erasure)
    fn as_bang(&self) -> Option<&dyn Any>;
    fn as_attr(&self) -> Option<&dyn Any>;
    fn as_legacy_bang(&self) -> Option<&dyn Any>;
    fn as_derive(&self) -> Option<&dyn Any>;
    fn as_legacy_derive(&self) -> Option<&dyn Any>;
    fn as_glob_delegation(&self) -> Option<&dyn Any>;
}
