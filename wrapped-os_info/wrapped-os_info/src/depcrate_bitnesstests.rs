// Generated macro for tests (module)
macro_rules! Depcrate_bitnesstests {
() => {
// Module: crate::bitness
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (test , any (target_os = "aix" , target_os = "dragonfly" , target_os = "freebsd" , target_os = "linux" , target_os = "macos" , target_os = "netbsd" , target_os = "openbsd" , target_os = "cygwin")))] mod tests { use super :: * ; use pretty_assertions :: assert_ne ; # [test] fn get_bitness () { let b = get () ; assert_ne ! (b , Bitness :: Unknown) ; } # [test] fn display () { let data = [(Bitness :: Unknown , "unknown bitness") , (Bitness :: X32 , "32-bit") , (Bitness :: X64 , "64-bit") ,] ; for (bitness , expected) in & data { assert_eq ! (& bitness . to_string () , expected) ; } } }
};
}
