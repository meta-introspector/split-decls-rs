// Generated macro for macro_95 (macro)
macro_rules! Depcratemacro_95 {
() => {
// Module: crate
// Provides: {"macro_95"}
// Dependencies: {}
bitflags ! { # [doc = " Flags controlling the behavior of ODB lookup operations"] # [derive (Clone , Copy , Debug , Eq , PartialEq , PartialOrd , Ord , Hash)] pub struct OdbLookupFlags : u32 { # [doc = " Don't call `git_odb_refresh` if the lookup fails. Useful when doing"] # [doc = " a batch of lookup operations for objects that may legitimately not"] # [doc = " exist. When using this flag, you may wish to manually call"] # [doc = " `git_odb_refresh` before processing a batch of objects."] const NO_REFRESH = raw :: GIT_ODB_LOOKUP_NO_REFRESH as u32 ; } }
};
}
