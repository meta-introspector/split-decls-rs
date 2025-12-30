// Generated macro for test (module)
macro_rules! Depcrate_options_versiontest {
() => {
// Module: crate::options::version
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: options :: { Options , OptionsResult } ; use std :: ffi :: OsStr ; # [test] fn version () { let args = vec ! [OsStr :: new ("--version")] ; let opts = Options :: parse (args , & None) ; assert ! (matches ! (opts , OptionsResult :: Version (_))) ; } # [test] fn version_with_file () { let args = vec ! [OsStr :: new ("--version") , OsStr :: new ("me")] ; let opts = Options :: parse (args , & None) ; assert ! (matches ! (opts , OptionsResult :: Version (_))) ; } }
};
}
