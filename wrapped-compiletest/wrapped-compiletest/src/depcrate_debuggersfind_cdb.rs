// Generated macro for find_cdb (function)
macro_rules! Depcrate_debuggersfind_cdb {
() => {
// Module: crate::debuggers
// Provides: {"find_cdb"}
// Dependencies: {}
# [doc = " FIXME: this is very questionable..."] fn find_cdb (target : & str) -> Option < Utf8PathBuf > { if ! (cfg ! (windows) && is_pc_windows_msvc_target (target)) { return None ; } let pf86 = Utf8PathBuf :: from_path_buf (env :: var_os ("ProgramFiles(x86)") . or_else (| | env :: var_os ("ProgramFiles")) ? . into () ,) . unwrap () ; let cdb_arch = if cfg ! (target_arch = "x86") { "x86" } else if cfg ! (target_arch = "x86_64") { "x64" } else if cfg ! (target_arch = "aarch64") { "arm64" } else if cfg ! (target_arch = "arm") { "arm" } else { return None ; } ; let mut path = pf86 ; path . push (r"Windows Kits\10\Debuggers") ; path . push (cdb_arch) ; path . push (r"cdb.exe") ; if ! path . exists () { return None ; } Some (path) }
};
}
