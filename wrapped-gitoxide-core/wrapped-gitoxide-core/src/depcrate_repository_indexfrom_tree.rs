// Generated macro for from_tree (function)
macro_rules! Depcrate_repository_indexfrom_tree {
() => {
// Module: crate::repository::index
// Provides: {"from_tree"}
// Dependencies: {}
pub fn from_tree (repo : gix :: Repository , mut spec : OsString , index_path : Option < PathBuf > , force : bool , skip_hash : bool ,) -> anyhow :: Result < () > { spec . push ("^{tree}") ; let spec = gix :: path :: os_str_into_bstr (& spec) ? ; let tree = repo . rev_parse_single (spec) ? ; let mut index = repo . index_from_tree (& tree) ? ; let options = gix :: index :: write :: Options { skip_hash , .. Default :: default () } ; match index_path { Some (index_path) => { if index_path . is_file () && ! force { anyhow :: bail ! ("File at \"{}\" already exists, to overwrite use the '-f' flag" , index_path . display ()) ; } index . set_path (index_path) ; index . write (options) ? ; } None => { let mut out = Vec :: with_capacity (512 * 1024) ; index . write_to (& mut out , options) ? ; } } Ok (()) }
};
}
