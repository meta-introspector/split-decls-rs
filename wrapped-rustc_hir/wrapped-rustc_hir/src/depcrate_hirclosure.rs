// Generated macro for Closure (struct)
macro_rules! Depcrate_hirClosure {
() => {
// Module: crate::hir
// Provides: {"Closure"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct Closure < 'hir > { pub def_id : LocalDefId , pub binder : ClosureBinder , pub constness : Constness , pub capture_clause : CaptureBy , pub bound_generic_params : & 'hir [GenericParam < 'hir >] , pub fn_decl : & 'hir FnDecl < 'hir > , pub body : BodyId , # [doc = " The span of the declaration block: 'move |...| -> ...'"] pub fn_decl_span : Span , # [doc = " The span of the argument block `|...|`"] pub fn_arg_span : Option < Span > , pub kind : ClosureKind , }
};
}
