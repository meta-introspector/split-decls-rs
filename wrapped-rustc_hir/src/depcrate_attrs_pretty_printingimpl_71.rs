// Generated macro for impl_71 (impl)
macro_rules! Depcrate_attrs_pretty_printingimpl_71 {
() => {
// Module: crate::attrs::pretty_printing
// Provides: {"impl_71"}
// Dependencies: {}
impl < T : PrintAttribute > PrintAttribute for Option < T > { fn should_render (& self) -> bool { self . as_ref () . is_some_and (| x | x . should_render ()) } fn print_attribute (& self , p : & mut Printer) { if let Some (i) = self { T :: print_attribute (i , p) } } }
};
}
