// Generated macro for build_langcall (function)
macro_rules! Depcrate_commonbuild_langcall {
() => {
// Module: crate::common
// Provides: {"build_langcall"}
// Dependencies: {}
pub (crate) fn build_langcall < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > (bx : & Bx , span : Span , li : LangItem ,) -> (Bx :: FnAbiOfResult , Bx :: Value , Instance < 'tcx >) { let tcx = bx . tcx () ; let def_id = tcx . require_lang_item (li , span) ; let instance = ty :: Instance :: mono (tcx , def_id) ; (bx . fn_abi_of_instance (instance , ty :: List :: empty ()) , bx . get_fn_addr (instance) , instance) }
};
}
