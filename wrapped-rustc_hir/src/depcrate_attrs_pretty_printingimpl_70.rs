// Generated macro for impl_70 (impl)
macro_rules! Depcrate_attrs_pretty_printingimpl_70 {
() => {
// Module: crate::attrs::pretty_printing
// Provides: {"impl_70"}
// Dependencies: {}
impl < T : PrintAttribute > PrintAttribute for & T { fn should_render (& self) -> bool { T :: should_render (self) } fn print_attribute (& self , p : & mut Printer) { T :: print_attribute (self , p) } }
};
}
