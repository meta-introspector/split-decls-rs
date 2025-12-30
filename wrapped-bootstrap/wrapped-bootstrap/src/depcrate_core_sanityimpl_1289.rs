// Generated macro for impl_1289 (impl)
macro_rules! Depcrate_core_sanityimpl_1289 {
() => {
// Module: crate::core::sanity
// Provides: {"impl_1289"}
// Dependencies: {}
impl Finder { pub fn new () -> Self { Self { cache : HashMap :: new () , path : env :: var_os ("PATH") . unwrap_or_default () } } pub fn maybe_have < S : Into < OsString > > (& mut self , cmd : S) -> Option < PathBuf > { let cmd : OsString = cmd . into () ; let path = & self . path ; self . cache . entry (cmd . clone ()) . or_insert_with (| | { for path in env :: split_paths (path) { let target = path . join (& cmd) ; let mut cmd_exe = cmd . clone () ; cmd_exe . push (".exe") ; if target . is_file () || path . join (& cmd_exe) . exists () || target . join (& cmd_exe) . exists () { return Some (target) ; } } None }) . clone () } pub fn must_have < S : AsRef < OsStr > > (& mut self , cmd : S) -> PathBuf { self . maybe_have (& cmd) . unwrap_or_else (| | { panic ! ("\n\ncouldn't find required command: {:?}\n\n" , cmd . as_ref ()) ; }) } }
};
}
