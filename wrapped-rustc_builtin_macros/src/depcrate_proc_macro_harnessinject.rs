// Generated macro for inject (function)
macro_rules! Depcrate_proc_macro_harnessinject {
() => {
// Module: crate::proc_macro_harness
// Provides: {"inject"}
// Dependencies: {}
pub fn inject (krate : & mut ast :: Crate , sess : & Session , features : & Features , resolver : & mut dyn ResolverExpand , is_proc_macro_crate : bool , has_proc_macro_decls : bool , is_test_crate : bool , dcx : DiagCtxtHandle < '_ > ,) { let ecfg = ExpansionConfig :: default (sym :: proc_macro , features) ; let mut cx = ExtCtxt :: new (sess , ecfg , resolver , None) ; let mut collect = CollectProcMacros { macros : Vec :: new () , in_root : true , dcx , session : sess , source_map : sess . source_map () , is_proc_macro_crate , is_test_crate , } ; if has_proc_macro_decls || is_proc_macro_crate { visit :: walk_crate (& mut collect , krate) ; } let macros = collect . macros ; if ! is_proc_macro_crate { return ; } if is_test_crate { return ; } let decls = mk_decls (& mut cx , & macros) ; krate . items . push (decls) ; }
};
}
