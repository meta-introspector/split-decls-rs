// Generated macro for impl_7399 (impl)
macro_rules! Depcrate_missing_enforced_import_renameimpl_7399 {
() => {
// Module: crate::missing_enforced_import_rename
// Provides: {"impl_7399"}
// Dependencies: {}
impl ImportRename { pub fn new (tcx : TyCtxt < '_ > , conf : & 'static Conf) -> Self { Self { renames : conf . enforced_import_renames . iter () . map (| x | (& x . path , Symbol :: intern (& x . rename))) . flat_map (| (path , rename) | { lookup_path_str (tcx , PathNS :: Arbitrary , path) . into_iter () . map (move | id | (id , rename)) }) . collect () , } } }
};
}
