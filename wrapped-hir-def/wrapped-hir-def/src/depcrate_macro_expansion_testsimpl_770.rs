// Generated macro for impl_770 (impl)
macro_rules! Depcrate_macro_expansion_testsimpl_770 {
() => {
// Module: crate::macro_expansion_tests
// Provides: {"impl_770"}
// Dependencies: {}
impl ProcMacroExpander for IdentityWhenValidProcMacroExpander { fn expand (& self , subtree : & TopSubtree , _ : Option < & TopSubtree > , _ : & base_db :: Env , _ : Span , _ : Span , _ : Span , _ : String ,) -> Result < TopSubtree , ProcMacroExpansionError > { let (parse , _) = syntax_bridge :: token_tree_to_syntax_node (subtree , syntax_bridge :: TopEntryPoint :: MacroItems , & mut | _ | span :: Edition :: CURRENT , span :: Edition :: CURRENT ,) ; if parse . errors () . is_empty () { Ok (subtree . clone ()) } else { panic ! ("got invalid macro input: {:?}" , parse . errors ()) ; } } fn eq_dyn (& self , other : & dyn ProcMacroExpander) -> bool { other . type_id () == TypeId :: of :: < Self > () } }
};
}
