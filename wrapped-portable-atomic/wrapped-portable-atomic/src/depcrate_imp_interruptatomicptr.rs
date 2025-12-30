// Generated macro for AtomicPtr (struct)
macro_rules! Depcrate_imp_interruptAtomicPtr {
() => {
// Module: crate::imp::interrupt
// Provides: {"AtomicPtr"}
// Dependencies: {}
# [cfg_attr (target_pointer_width = "16" , repr (C , align (2)))] # [cfg_attr (target_pointer_width = "32" , repr (C , align (4)))] # [cfg_attr (target_pointer_width = "64" , repr (C , align (8)))] # [cfg_attr (target_pointer_width = "128" , repr (C , align (16)))] pub (crate) struct AtomicPtr < T > { p : UnsafeCell < * mut T > , }
};
}
