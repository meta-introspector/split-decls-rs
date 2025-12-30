// Generated macro for impl_232 (impl)
macro_rules! Depcrate_core_build_steps_distimpl_232 {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"impl_232"}
// Dependencies: {}
impl Step for DebuggerScripts { type Output = () ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . never () } fn run (self , builder : & Builder < '_ >) { let target = self . target ; let sysroot = self . sysroot ; let dst = sysroot . join ("lib/rustlib/etc") ; t ! (fs :: create_dir_all (& dst)) ; let cp_debugger_script = | file : & str | { builder . install (& builder . src . join ("src/etc/") . join (file) , & dst , FileType :: Regular) ; } ; if target . contains ("windows-msvc") { builder . install (& builder . src . join ("src/etc/rust-windbg.cmd") , & sysroot . join ("bin") , FileType :: Script ,) ; cp_debugger_script ("natvis/intrinsic.natvis") ; cp_debugger_script ("natvis/liballoc.natvis") ; cp_debugger_script ("natvis/libcore.natvis") ; cp_debugger_script ("natvis/libstd.natvis") ; } cp_debugger_script ("rust_types.py") ; builder . install (& builder . src . join ("src/etc/rust-gdb") , & sysroot . join ("bin") , FileType :: Script ,) ; builder . install (& builder . src . join ("src/etc/rust-gdbgui") , & sysroot . join ("bin") , FileType :: Script ,) ; cp_debugger_script ("gdb_load_rust_pretty_printers.py") ; cp_debugger_script ("gdb_lookup.py") ; cp_debugger_script ("gdb_providers.py") ; builder . install (& builder . src . join ("src/etc/rust-lldb") , & sysroot . join ("bin") , FileType :: Script ,) ; cp_debugger_script ("lldb_lookup.py") ; cp_debugger_script ("lldb_providers.py") ; cp_debugger_script ("lldb_commands") } }
};
}
