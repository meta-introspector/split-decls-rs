// Generated macro for impl_72 (impl)
macro_rules! Depcrate_attrs_pretty_printingimpl_72 {
() => {
// Module: crate::attrs::pretty_printing
// Provides: {"impl_72"}
// Dependencies: {}
impl < T : PrintAttribute > PrintAttribute for ThinVec < T > { fn should_render (& self) -> bool { self . is_empty () || self [0] . should_render () } fn print_attribute (& self , p : & mut Printer) { let mut last_printed = false ; p . word ("[") ; for i in self { if last_printed { p . word_space (",") ; } i . print_attribute (p) ; last_printed = i . should_render () ; } p . word ("]") ; } }
};
}
