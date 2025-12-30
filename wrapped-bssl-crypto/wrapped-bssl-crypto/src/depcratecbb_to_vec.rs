// Generated macro for cbb_to_vec (function)
macro_rules! Depcratecbb_to_vec {
() => {
// Module: crate
// Provides: {"cbb_to_vec"}
// Dependencies: {}
# [cfg (feature = "mlalgs")] # [doc = " Calls `func` with a `CBB` pointer that has been initialized to a vector"] # [doc = " of `len` bytes. That function must write exactly `len` bytes to the"] # [doc = " `CBB`. Those bytes are then returned as a vector."] # [allow (clippy :: unwrap_used)] fn cbb_to_vec < F : FnOnce (* mut bssl_sys :: CBB) > (len : usize , func : F) -> Vec < u8 > { let mut boxed = Box :: new_uninit_slice (len) ; let mut cbb = unsafe { initialized_struct_fallible (| cbb | { bssl_sys :: CBB_init_fixed (cbb , boxed . as_mut_ptr () as * mut u8 , len) == 1 }) } . unwrap () ; func (& mut cbb) ; unsafe { assert_eq ! (bssl_sys :: CBB_len (& cbb) , len) ; boxed . assume_init () . into () } }
};
}
