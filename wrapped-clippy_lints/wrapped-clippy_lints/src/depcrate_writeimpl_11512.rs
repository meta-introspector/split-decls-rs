// Generated macro for impl_11512 (impl)
macro_rules! Depcrate_writeimpl_11512 {
() => {
// Module: crate::write
// Provides: {"impl_11512"}
// Dependencies: {}
impl Write { pub fn new (conf : & 'static Conf , format_args : FormatArgsStorage) -> Self { Self { format_args , outermost_debug_impl : None , allow_print_in_tests : conf . allow_print_in_tests , } } fn in_debug_impl (& self) -> bool { self . outermost_debug_impl . is_some () } }
};
}
