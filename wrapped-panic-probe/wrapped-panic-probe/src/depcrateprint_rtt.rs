// Generated macro for print_rtt (module)
macro_rules! Depcrateprint_rtt {
() => {
// Module: crate
// Provides: {"print_rtt"}
// Dependencies: {}
# [cfg (feature = "print-rtt")] mod print_rtt { use core :: panic :: PanicInfo ; use rtt_target :: rprintln ; pub fn print (info : & PanicInfo) { rprintln ! ("{}" , info) ; } }
};
}
