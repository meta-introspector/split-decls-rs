// Generated macro for print_defmt (module)
macro_rules! Depcrateprint_defmt {
() => {
// Module: crate
// Provides: {"print_defmt"}
// Dependencies: {}
# [cfg (feature = "print-defmt")] mod print_defmt { use core :: panic :: PanicInfo ; pub fn print (info : & PanicInfo) { defmt :: error ! ("{}" , defmt :: Display2Format (info)) ; } }
};
}
