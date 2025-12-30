// Generated macro for imp (module)
macro_rules! Depcrateimp {
() => {
// Module: crate
// Provides: {"imp"}
// Dependencies: {}
# [cfg (target_os = "none")] mod imp { use core :: panic :: PanicInfo ; use core :: sync :: atomic :: { AtomicBool , Ordering } ; # [cfg (feature = "print-rtt")] use crate :: print_rtt :: print ; # [cfg (feature = "print-defmt")] use crate :: print_defmt :: print ; # [cfg (not (any (feature = "print-rtt" , feature = "print-defmt")))] fn print (_ : & core :: panic :: PanicInfo) { } # [panic_handler] fn panic (info : & PanicInfo) -> ! { static PANICKED : AtomicBool = AtomicBool :: new (false) ; cortex_m :: interrupt :: disable () ; if ! PANICKED . load (Ordering :: Relaxed) { PANICKED . store (true , Ordering :: Relaxed) ; print (info) ; } crate :: hard_fault () ; } }
};
}
