// Generated macro for print_skip (macro)
macro_rules! Depcrate_attrs_pretty_printingprint_skip {
() => {
// Module: crate::attrs::pretty_printing
// Provides: {"print_skip"}
// Dependencies: {}
macro_rules ! print_skip { ($ ($ t : ty) ,* $ (,) ?) => { $ (impl PrintAttribute for $ t { fn should_render (& self) -> bool { false } fn print_attribute (& self , _ : & mut Printer) { } }) * } ; }
};
}
