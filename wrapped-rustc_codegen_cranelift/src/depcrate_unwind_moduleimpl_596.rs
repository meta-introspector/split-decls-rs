// Generated macro for impl_596 (impl)
macro_rules! Depcrate_unwind_moduleimpl_596 {
() => {
// Module: crate::unwind_module
// Provides: {"impl_596"}
// Dependencies: {}
impl < T : Module > UnwindModule < T > { pub (crate) fn new (mut module : T , pic_eh_frame : bool) -> Self { let unwind_context = UnwindContext :: new (& mut module , pic_eh_frame) ; UnwindModule { module , unwind_context } } }
};
}
