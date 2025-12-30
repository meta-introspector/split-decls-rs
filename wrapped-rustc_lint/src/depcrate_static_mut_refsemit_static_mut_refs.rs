// Generated macro for emit_static_mut_refs (function)
macro_rules! Depcrate_static_mut_refsemit_static_mut_refs {
() => {
// Module: crate::static_mut_refs
// Provides: {"emit_static_mut_refs"}
// Dependencies: {}
fn emit_static_mut_refs (cx : & LateContext < '_ > , span : Span , sugg_span : Span , mutable : Mutability , suggest_addr_of : bool ,) { let (shared_label , shared_note , mut_note , sugg) = match mutable { Mutability :: Mut => { let sugg = if suggest_addr_of { Some (MutRefSugg :: Mut { span : sugg_span }) } else { None } ; ("mutable " , false , true , sugg) } Mutability :: Not => { let sugg = if suggest_addr_of { Some (MutRefSugg :: Shared { span : sugg_span }) } else { None } ; ("shared " , true , false , sugg) } } ; cx . emit_span_lint (STATIC_MUT_REFS , span , RefOfMutStatic { span , sugg , shared_label , shared_note , mut_note } ,) ; }
};
}
