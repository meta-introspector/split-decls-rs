// Generated macro for print (function)
macro_rules! Depcrate_readobjprint {
() => {
// Module: crate::readobj
// Provides: {"print"}
// Dependencies: {}
pub fn print (w : & mut dyn Write , e : & mut dyn Write , file : & [u8] , extra_files : & [& [u8]] , options : & PrintOptions ,) { let mut printer = Printer :: new (w , e , options) ; print_object (& mut printer , file , extra_files) ; }
};
}
