// Generated macro for impl_523 (impl)
macro_rules! Depcrate_srcimpl_523 {
() => {
// Module: crate::src
// Provides: {"impl_523"}
// Dependencies: {}
impl HasChildSource < LocalLifetimeParamId > for GenericDefId { type Value = ast :: LifetimeParam ; fn child_source (& self , db : & dyn DefDatabase ,) -> InFile < ArenaMap < LocalLifetimeParamId , Self :: Value > > { let generic_params = db . generic_params (* self) ; let idx_iter = generic_params . iter_lt () . map (| (idx , _) | idx) ; let (file_id , generic_params_list) = self . file_id_and_params_of (db) ; let mut params = ArenaMap :: default () ; if let Some (generic_params_list) = generic_params_list { for (idx , ast_param) in idx_iter . zip (generic_params_list . lifetime_params ()) { params . insert (idx , ast_param) ; } } InFile :: new (file_id , params) } }
};
}
