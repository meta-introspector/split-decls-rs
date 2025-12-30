// Generated macro for macro_707 (macro)
macro_rules! Depcratemacro_707 {
() => {
// Module: crate
// Provides: {"macro_707"}
// Dependencies: {}
cfg_has_atomic_ptr ! { # [cfg (target_pointer_width = "16")] atomic_int ! (AtomicIsize , isize , 2 , cfg_has_atomic_cas_or_amo8 , cfg_no_atomic_cas_or_amo8) ; # [cfg (target_pointer_width = "16")] atomic_int ! (AtomicUsize , usize , 2 , cfg_has_atomic_cas_or_amo8 , cfg_no_atomic_cas_or_amo8) ; # [cfg (target_pointer_width = "32")] atomic_int ! (AtomicIsize , isize , 4 , cfg_has_atomic_cas_or_amo32 , cfg_no_atomic_cas_or_amo32) ; # [cfg (target_pointer_width = "32")] atomic_int ! (AtomicUsize , usize , 4 , cfg_has_atomic_cas_or_amo32 , cfg_no_atomic_cas_or_amo32) ; # [cfg (target_pointer_width = "64")] atomic_int ! (AtomicIsize , isize , 8 , cfg_has_atomic_cas_or_amo32 , cfg_no_atomic_cas_or_amo32) ; # [cfg (target_pointer_width = "64")] atomic_int ! (AtomicUsize , usize , 8 , cfg_has_atomic_cas_or_amo32 , cfg_no_atomic_cas_or_amo32) ; # [cfg (target_pointer_width = "128")] atomic_int ! (AtomicIsize , isize , 16 , cfg_has_atomic_cas_or_amo32 , cfg_no_atomic_cas_or_amo32) ; # [cfg (target_pointer_width = "128")] atomic_int ! (AtomicUsize , usize , 16 , cfg_has_atomic_cas_or_amo32 , cfg_no_atomic_cas_or_amo32) ; }
};
}
