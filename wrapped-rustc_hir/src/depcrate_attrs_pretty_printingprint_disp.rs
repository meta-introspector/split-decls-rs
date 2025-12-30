// Generated macro for print_disp (macro)
macro_rules! Depcrate_attrs_pretty_printingprint_disp {
() => {
// Module: crate::attrs::pretty_printing
// Provides: {"print_disp"}
// Dependencies: {}
macro_rules ! print_disp { ($ ($ t : ty) ,* $ (,) ?) => { $ (impl PrintAttribute for $ t { fn should_render (& self) -> bool { true } fn print_attribute (& self , p : & mut Printer) { p . word (format ! ("{}" , self)) ; } }) * } ; }
};
}
