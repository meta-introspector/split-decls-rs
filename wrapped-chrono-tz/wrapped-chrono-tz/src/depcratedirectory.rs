// Generated macro for directory (module)
macro_rules! Depcratedirectory {
() => {
// Module: crate
// Provides: {"directory"}
// Dependencies: {}
# [cfg (any (feature = "case-insensitive" , feature = "filter-by-regex"))] mod directory { # ! [allow (dead_code , non_camel_case_types , non_snake_case , non_upper_case_globals)] include ! (concat ! (env ! ("OUT_DIR") , "/directory.rs")) ; }
};
}
