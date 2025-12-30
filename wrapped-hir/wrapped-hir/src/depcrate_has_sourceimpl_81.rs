// Generated macro for impl_81 (impl)
macro_rules! Depcrate_has_sourceimpl_81 {
() => {
// Module: crate::has_source
// Provides: {"impl_81"}
// Dependencies: {}
impl HasSource for SelfParam { type Ast = ast :: SelfParam ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { let InFile { file_id , value } = Function :: from (self . func) . source (db) ? ; value . param_list () . and_then (| params | params . self_param ()) . map (| value | InFile { file_id , value }) } }
};
}
