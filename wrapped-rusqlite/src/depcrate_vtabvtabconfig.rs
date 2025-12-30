// Generated macro for VTabConfig (enum)
macro_rules! Depcrate_vtabVTabConfig {
() => {
// Module: crate::vtab
// Provides: {"VTabConfig"}
// Dependencies: {}
# [doc = " Virtual table configuration options"] # [repr (i32)] # [non_exhaustive] # [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum VTabConfig { # [doc = " Equivalent to `SQLITE_VTAB_CONSTRAINT_SUPPORT`"] ConstraintSupport = 1 , # [doc = " Equivalent to `SQLITE_VTAB_INNOCUOUS`"] Innocuous = 2 , # [doc = " Equivalent to `SQLITE_VTAB_DIRECTONLY`"] DirectOnly = 3 , # [doc = " Equivalent to `SQLITE_VTAB_USES_ALL_SCHEMAS`"] UsesAllSchemas = 4 , }
};
}
