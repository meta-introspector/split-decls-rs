// Generated macro for initd (function)
macro_rules! Depcrateinitd {
() => {
// Module: crate
// Provides: {"initd"}
// Dependencies: {}
# [doc = " Entry point of a kernel thread, which initialize the libos"] # [cfg (target_os = "none")] extern "C" fn initd (_arg : usize) { unsafe extern "C" { # [cfg (all (not (test) , not (any (feature = "nostd" , feature = "common-os"))))] fn runtime_entry (argc : i32 , argv : * const * const u8 , env : * const * const u8) -> ! ; # [cfg (all (not (test) , any (feature = "nostd" , feature = "common-os")))] fn main (argc : i32 , argv : * const * const u8 , env : * const * const u8) ; } if env :: is_uhyve () { info ! ("Hermit is running on uhyve!") ; } else { info ! ("Hermit is running on common system!") ; } drivers :: init () ; crate :: executor :: init () ; syscalls :: init () ; fs :: init () ; # [cfg (feature = "shell")] shell :: init () ; # [cfg (not (test))] let (argc , argv , environ) = syscalls :: get_application_parameters () ; core_scheduler () . reschedule () ; info ! ("Jumping into application") ; # [cfg (not (test))] unsafe { # [cfg (all (not (test) , not (any (feature = "nostd" , feature = "common-os"))))] runtime_entry (argc , argv , environ) ; # [cfg (all (not (test) , any (feature = "nostd" , feature = "common-os")))] main (argc , argv , environ) ; } # [cfg (test)] test_main () ; }
};
}
