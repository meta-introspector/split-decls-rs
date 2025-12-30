// Generated macro for impl_22 (impl)
macro_rules! Depcrate_elisionimpl_22 {
() => {
// Module: crate::elision
// Provides: {"impl_22"}
// Dependencies: {}
# [cfg (not (all (feature = "hardware-lock-elision" , not (miri) , any (target_arch = "x86" , target_arch = "x86_64"))))] impl AtomicElisionExt for AtomicUsize { type IntType = usize ; # [inline] fn elision_compare_exchange_acquire (& self , _ : usize , _ : usize) -> Result < usize , usize > { unreachable ! () ; } # [inline] fn elision_fetch_sub_release (& self , _ : usize) -> usize { unreachable ! () ; } }
};
}
