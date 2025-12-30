// Generated macro for print_debug (macro)
macro_rules! Depcrate_attrs_pretty_printingprint_debug {
() => {
// Module: crate::attrs::pretty_printing
// Provides: {"print_debug"}
// Dependencies: {}
macro_rules ! print_debug { ($ ($ t : ty) ,* $ (,) ?) => { $ (impl PrintAttribute for $ t { fn should_render (& self) -> bool { true } fn print_attribute (& self , p : & mut Printer) { p . word (format ! ("{:?}" , self)) ; } }) * } ; }
};
}
