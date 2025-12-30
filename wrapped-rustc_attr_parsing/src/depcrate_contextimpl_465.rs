// Generated macro for impl_465 (impl)
macro_rules! Depcrate_contextimpl_465 {
() => {
// Module: crate::context
// Provides: {"impl_465"}
// Dependencies: {}
impl < 'f , 'sess : 'f , S : Stage > SharedContext < 'f , 'sess , S > { pub (crate) fn emit_err (& self , diag : impl for < 'x > Diagnostic < 'x >) -> ErrorGuaranteed { self . stage . emit_err (& self . sess , diag) } # [doc = " Emit a lint. This method is somewhat special, since lints emitted during attribute parsing"] # [doc = " must be delayed until after HIR is built. This method will take care of the details of"] # [doc = " that."] pub (crate) fn emit_lint (& mut self , lint : AttributeLintKind , span : Span) { if ! matches ! (self . stage . should_emit () , ShouldEmit :: ErrorsAndLints | ShouldEmit :: EarlyFatal { also_emit_lints : true }) { return ; } let id = self . target_id ; (self . emit_lint) (AttributeLint { id , span , kind : lint }) ; } pub (crate) fn warn_unused_duplicate (& mut self , used_span : Span , unused_span : Span) { self . emit_lint (AttributeLintKind :: UnusedDuplicate { this : unused_span , other : used_span , warning : false , } , unused_span ,) } pub (crate) fn warn_unused_duplicate_future_error (& mut self , used_span : Span , unused_span : Span ,) { self . emit_lint (AttributeLintKind :: UnusedDuplicate { this : unused_span , other : used_span , warning : true , } , unused_span ,) } }
};
}
