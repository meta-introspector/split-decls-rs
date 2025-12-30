// Generated macro for impl_26 (impl)
macro_rules! Depcrate_corpus_dbimpl_26 {
() => {
// Module: crate::corpus::db
// Provides: {"impl_26"}
// Dependencies: {}
impl Repo { pub (crate) fn try_from (repo : & gix :: Repository) -> anyhow :: Result < Self > { let num_references = repo . refs . iter () ? . all () ? . count () ; let num_objects = repo . objects . packed_object_count () ? ; let odb_size = ByteSize (std :: fs :: read_dir (repo . objects . store_ref () . path () . join ("pack")) . map (| dir | { dir . filter_map (Result :: ok) . filter_map (| e | e . metadata () . ok ()) . filter_map (| m | m . is_file () . then_some (m . len ())) . sum () }) . unwrap_or_default () ,) ; Ok (Repo { id : 0 , path : repo . path () . to_owned () , odb_size , num_objects , num_references , }) } }
};
}
