// Generated macro for build_CBS (function)
macro_rules! Depcrate_cbsbuild_CBS {
() => {
// Module: crate::cbs
// Provides: {"build_CBS"}
// Dependencies: {}
# [inline] # [allow (non_snake_case)] pub fn build_CBS (data : & [u8]) -> CBS { let mut cbs = MaybeUninit :: < CBS > :: uninit () ; unsafe { CBS_init (cbs . as_mut_ptr () , data . as_ptr () , data . len ()) } ; unsafe { cbs . assume_init () } }
};
}
