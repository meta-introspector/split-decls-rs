// Generated macro for pkg_loc (function)
macro_rules! Depcrate_helperspkg_loc {
() => {
// Module: crate::helpers
// Provides: {"pkg_loc"}
// Dependencies: {}
pub fn pkg_loc (name : & str , loc : & str) -> Summary { let link = if name . ends_with ("-sys") { Some (name) } else { None } ; Summary :: new (pkg_id_loc (name , loc) , Vec :: new () , & BTreeMap :: new () , link , None ,) . unwrap () }
};
}
