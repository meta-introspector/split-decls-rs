// Generated macro for sanitize_sh (function)
macro_rules! Depcrate_core_build_steps_installsanitize_sh {
() => {
// Module: crate::core::build_steps::install
// Provides: {"sanitize_sh"}
// Dependencies: {}
# [doc = " We have to run a few shell scripts, which choke quite a bit on both `\\`"] # [doc = " characters and on `C:\\` paths, so normalize both of them away."] fn sanitize_sh (path : & Path , is_cygwin : bool) -> String { let path = path . to_str () . unwrap () . replace ('\\' , "/") ; return if is_cygwin { path } else { change_drive (unc_to_lfs (& path)) . unwrap_or (path) } ; fn unc_to_lfs (s : & str) -> & str { s . strip_prefix ("//?/") . unwrap_or (s) } fn change_drive (s : & str) -> Option < String > { let mut ch = s . chars () ; let drive = ch . next () . unwrap_or ('C') ; if ch . next () != Some (':') { return None ; } if ch . next () != Some ('/') { return None ; } Some (format ! ("/proc/cygdrive/{}/{}" , drive , & s [drive . len_utf8 () + 2 ..])) } }
};
}
