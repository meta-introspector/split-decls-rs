// Generated macro for find_std_module (function)
macro_rules! Depcrate_defsfind_std_module {
() => {
// Module: crate::defs
// Provides: {"find_std_module"}
// Dependencies: {}
pub fn find_std_module (famous_defs : & FamousDefs < '_ , '_ > , name : & str , edition : Edition ,) -> Option < hir :: Module > { let db = famous_defs . 0 . db ; let std_crate = famous_defs . std () ? ; let std_root_module = std_crate . root_module () ; std_root_module . children (db) . find (| module | { module . name (db) . is_some_and (| module | module . display (db , edition) . to_string () == name) }) }
};
}
