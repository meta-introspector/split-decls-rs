// Generated macro for atomic_cmpxchg (macro)
macro_rules! Depcrate_arm_linuxatomic_cmpxchg {
() => {
// Module: crate::arm_linux
// Provides: {"atomic_cmpxchg"}
// Dependencies: {}
macro_rules ! atomic_cmpxchg { ($ name : ident , $ ty : ty) => { intrinsics ! { pub unsafe extern "C" fn $ name (ptr : * mut $ ty , oldval : $ ty , newval : $ ty) -> $ ty { unsafe { atomic_cmpxchg (ptr , oldval as u32 , newval as u32) as $ ty } } } } ; }
};
}
