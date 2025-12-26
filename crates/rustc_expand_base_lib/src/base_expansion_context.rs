//! This module defines the `BaseExpansionContext` trait, which serves as a minimal interface
//! for core macro expansion functionalities needed by `rustc_expand_base_lib`.
//!
//! For n00bs: The compiler's main context object (`ExtCtxt`) is like a huge toolbox with every
//! tool imaginable. But often, individual small tasks only need a few specific tools.
//! `BaseExpansionContext` is a "mini-contract" that says, "If you want to do basic macro stuff
//! in this library, you just need to promise to provide *these few* essential tools."
//!
//! This approach helps to:
//! - **Decouple:** `rustc_expand_base_lib` (our core toolbox) doesn't need to know about
//!   every single tool in the main `ExtCtxt`. It only cares about the ones it uses.
//! - **Prevent Monoliths:** We avoid moving the entire, complex `ExtCtxt` into the base library,
//!   keeping the base library focused and lightweight.
//! - **Improve Clarity:** It clearly defines the minimal requirements for a macro expansion
//!   context in this fundamental layer.
//!
//! The actual `ExtCtxt` from `rustc_expand` will then implement this trait, fulfilling the
//! contract and providing the necessary functionality.

use std::rc::Rc;
use std::path::PathBuf;
use std::sync::Arc;
use std::any::Any; // Added
use rustc_span::{Ident, LocalExpnId, Span, Symbol, DUMMY_SP};
use rustc_span::def_id::LocalDefId;
use rustc_errors::{DiagCtxtHandle, ErrorGuaranteed};
use rustc_ast::{self as ast, NodeId};
use rustc_feature::Features;
use rustc_hir::limit::Limit;

// Import types defined within rustc_expand_base_lib itself
use crate::prelude::{ExpansionData, ModuleData, DirOwnership, AstFragment, Invocation};
use super::resolver_traits::{DeriveResolutionProvider, ImportResolver};
use crate::resolver_traits::OpaqueDeriveResolution;

/// A trait that defines the minimal set of functionalities required from an expansion context
/// by the core components within `rustc_expand_base_lib`.
///
/// Any full expansion context (like `ExtCtxt` in `rustc_expand`) must implement this trait
/// to interact with `rustc_expand_base_lib`'s functionalities.
pub trait BaseExpansionContext<DRT: OpaqueDeriveResolution + 'static> {
    /// Returns a diagnostic context handle for emitting errors and warnings.
    fn dcx(&self) -> DiagCtxtHandle;

    /// Returns the current expansion data.
    fn current_expansion(&self) -> &ExpansionData;

    /// Returns a mutable reference to the current expansion data.
    fn current_expansion_mut(&mut self) -> &mut ExpansionData;

    /// Returns the expansion configuration.
    fn ecfg(&self) -> &ExpansionConfig;

    /// Returns a mutable reference to the resolver.
    fn resolver(&mut self) -> &mut (dyn ResolverExpand<DRT> + 'static);

    /// Returns the compiler session's features.
    fn features(&self) -> &Features;

    /// Emits an error when the recursion limit is reached.
    fn emit_recursion_limit_reached(&mut self, span: Span, descr: &str, suggested_limit: Limit, crate_name: Symbol) -> ErrorGuaranteed;

    /// Emits an error for wrong fragment kind.
    fn emit_wrong_fragment_kind(&mut self, span: Span, kind: &str, name_path: &ast::Path) -> ErrorGuaranteed;

    /// Increments macro error count and traces diagnostics.
    fn macro_error_and_trace_macros_diag(&mut self);

    /// Gets a new NodeId from the resolver.
    fn next_node_id(&mut self) -> NodeId;

    /// Records the name of an opaque `Ty::ImplTrait` pre-expansion.
    fn insert_impl_trait_name(&mut self, id: NodeId, name: Symbol);

    /// Resolves `$crate`s in the fragment for pretty-printing.
    fn resolve_dollar_crates(&mut self);

    /// Marks an invocation id as a glob delegation.
    fn register_glob_delegation(&mut self, invoc_id: LocalExpnId);

    /// Returns the parent `LocalDefId` for an invocation.
    fn invocation_parent(&self, id: LocalExpnId) -> LocalDefId; // Reverted to LocalDefId

    /// Returns span for the macro which originally caused the current expansion to happen.
    fn expansion_cause(&self) -> Option<Span>;
}

pub struct ExpansionConfig<'a> { // Re-added ExpansionConfig
    pub crate_name: Symbol,
    pub features: &'a Features,
    pub recursion_limit: Limit,
    pub trace_mac: bool,
    pub should_test: bool,
    pub span_debug: bool,
    pub proc_macro_backtrace: bool,
}

impl<'a> ExpansionConfig<'a> {
    pub fn default(crate_name: Symbol, features: &'a Features) -> ExpansionConfig<'a> {
        ExpansionConfig {
            crate_name,
            features,
            recursion_limit: Limit::new(1024),
            trace_mac: false,
            should_test: false,
            span_debug: false,
            proc_macro_backtrace: false,
        }
    }
}



// Placeholder for ResolverExpand. This trait will also need to be defined
// either here or its required methods moved here.
pub trait ResolverExpand<DRT: OpaqueDeriveResolution + 'static>: DeriveResolutionProvider<DRT> + ImportResolver {
    fn next_node_id(&mut self) -> NodeId;
    fn invocation_parent(&self, id: LocalExpnId) -> LocalDefId; // Corrected return type to LocalDefId
    fn resolve_dollar_crates(&mut self);
    // This is still tied to AstFragment, so AstFragment must be accessible.
    fn visit_ast_fragment_with_placeholders(&mut self, expn_id: LocalExpnId, fragment: &AstFragment);
    fn register_builtin_macro(&mut self, name: Symbol, ext: Arc<dyn Any + Send + Sync>); // Changed to use Any for flexibility
    fn resolve_macro_invocation(
        &mut self,
        invoc: &Invocation, // Invocation needs to be accessible
        eager_expansion_root: LocalExpnId,
        force: bool,
    ) -> Result<Arc<dyn Any + Send + Sync>, ErrorGuaranteed>; // Changed to use Any for flexibility, and ErrorGuaranteed
    fn insert_impl_trait_name(&mut self, id: NodeId, name: Symbol);
    fn register_glob_delegation(&mut self, invoc_id: LocalExpnId);
}

