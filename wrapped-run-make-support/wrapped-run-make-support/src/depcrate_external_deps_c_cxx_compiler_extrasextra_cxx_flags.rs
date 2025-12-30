// Generated macro for extra_cxx_flags (function)
macro_rules! Depcrate_external_deps_c_cxx_compiler_extrasextra_cxx_flags {
() => {
// Module: crate::external_deps::c_cxx_compiler::extras
// Provides: {"extra_cxx_flags"}
// Dependencies: {}
# [doc = " `EXTRACXXFLAGS`"] pub fn extra_cxx_flags () -> Vec < & 'static str > { if is_windows () { if is_windows_msvc () { vec ! [] } else { vec ! ["-lstdc++"] } } else { match & uname () [..] { "Darwin" => vec ! ["-lc++"] , "FreeBSD" | "SunOS" | "OpenBSD" => vec ! [] , _ => vec ! ["-lstdc++"] , } } }
};
}
