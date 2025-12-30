// Generated macro for application_processor_main (function)
macro_rules! Depcrateapplication_processor_main {
() => {
// Module: crate
// Provides: {"application_processor_main"}
// Dependencies: {}
# [doc = " Entry Point of Hermit for an Application Processor"] # [cfg (all (target_os = "none" , feature = "smp"))] fn application_processor_main () -> ! { arch :: application_processor_init () ; # [cfg (not (target_arch = "riscv64"))] scheduler :: add_current_core () ; interrupts :: enable () ; arch :: kernel :: boot_next_processor () ; debug ! ("Entering idle loop for application processor") ; synch_all_cores () ; crate :: executor :: init () ; PerCoreScheduler :: run () ; }
};
}
