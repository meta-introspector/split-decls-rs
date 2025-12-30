// Generated macro for _ (const)
macro_rules! Depcrate_utils_ {
() => {
// Module: crate::utils
// Provides: {"_"}
// Dependencies: {}
# [cfg (not (portable_atomic_no_asm_maybe_uninit))] # [cfg (target_pointer_width = "32")] const _ : () = assert ! (unsafe { zero_extend64_ptr (ptr :: without_provenance_mut (! 0)) . assume_init () == ! 0_u32 as u64 }) ;
};
}
