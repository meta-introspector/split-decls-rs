// Generated macro for impl_742 (impl)
macro_rules! Depcrate_ir_extnameimpl_742 {
() => {
// Module: crate::ir::extname
// Provides: {"impl_742"}
// Dependencies: {}
impl < 'a > fmt :: Display for DisplayableExternalName < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match & self . name { ExternalName :: User (func_ref) => { if let Some (params) = self . params { let name = & params . user_named_funcs () [* func_ref] ; write ! (f , "u{}:{}" , name . namespace , name . index) } else { write ! (f , "{}" , * func_ref) } } ExternalName :: TestCase (testcase) => testcase . fmt (f) , ExternalName :: LibCall (lc) => write ! (f , "%{lc}") , ExternalName :: KnownSymbol (ks) => write ! (f , "%{ks}") , } } }
};
}
