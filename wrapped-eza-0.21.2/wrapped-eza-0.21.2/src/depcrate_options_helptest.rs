// Generated macro for test (module)
macro_rules! Depcrate_options_helptest {
() => {
// Module: crate::options::help
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: options :: { Options , OptionsResult } ; use std :: ffi :: OsStr ; # [test] fn help () { let args = vec ! [OsStr :: new ("--help")] ; let opts = Options :: parse (args , & None) ; assert ! (matches ! (opts , OptionsResult :: Help (_))) ; } # [test] fn help_with_file () { let args = vec ! [OsStr :: new ("--help") , OsStr :: new ("me")] ; let opts = Options :: parse (args , & None) ; assert ! (matches ! (opts , OptionsResult :: Help (_))) ; } # [test] fn unhelpful () { let args = vec ! [] ; let opts = Options :: parse (args , & None) ; assert ! (! matches ! (opts , OptionsResult :: Help (_))) ; } }
};
}
