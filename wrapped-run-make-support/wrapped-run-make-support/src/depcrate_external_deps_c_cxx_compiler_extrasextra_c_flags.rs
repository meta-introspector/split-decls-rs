// Generated macro for extra_c_flags (function)
macro_rules! Depcrate_external_deps_c_cxx_compiler_extrasextra_c_flags {
() => {
// Module: crate::external_deps::c_cxx_compiler::extras
// Provides: {"extra_c_flags"}
// Dependencies: {}
# [doc = " `EXTRACFLAGS`"] pub fn extra_c_flags () -> Vec < & 'static str > { if is_windows () { if is_windows_msvc () { let mut args = get_windows_msvc_libs () ; if is_arm64ec () { args . push ("/arm64EC") ; } args } else { vec ! ["-lws2_32" , "-luserenv" , "-lbcrypt" , "-lntdll" , "-lsynchronization"] } } else { match uname () { n if n . contains ("Darwin") => vec ! ["-lresolv"] , n if n . contains ("FreeBSD") => vec ! ["-lm" , "-lpthread" , "-lgcc_s"] , n if n . contains ("SunOS") => { vec ! ["-lm" , "-lpthread" , "-lposix4" , "-lsocket" , "-lresolv"] } n if n . contains ("OpenBSD") => vec ! ["-lm" , "-lpthread" , "-lc++abi"] , _ => vec ! ["-lm" , "-lrt" , "-ldl" , "-lpthread"] , } } }
};
}
