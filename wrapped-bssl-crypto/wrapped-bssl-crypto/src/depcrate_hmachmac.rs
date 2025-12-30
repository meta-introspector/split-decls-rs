// Generated macro for hmac (function)
macro_rules! Depcrate_hmachmac {
() => {
// Module: crate::hmac
// Provides: {"hmac"}
// Dependencies: {}
# [doc = " Private generically implemented function for computing HMAC as a oneshot operation."] # [doc = " This should only be exposed publicly by types with the correct output size `N` which corresponds"] # [doc = " to the output size of the provided generic hash function. Ideally `N` would just come from `MD`,"] # [doc = " but this is not possible until the Rust language can support the `min_const_generics` feature."] # [doc = " Until then we will have to pass both separately: https://github.com/rust-lang/rust/issues/60551"] # [inline] fn hmac < const N : usize , MD : digest :: Algorithm > (key : & [u8] , data : & [u8]) -> [u8 ; N] { let mut out = [0_u8 ; N] ; let mut size : c_uint = 0 ; let result = unsafe { bssl_sys :: HMAC (MD :: get_md (sealed :: Sealed) . as_ptr () , key . as_ffi_void_ptr () , key . len () , data . as_ffi_ptr () , data . len () , out . as_mut_ffi_ptr () , & mut size as * mut c_uint ,) } ; assert_eq ! (size as usize , N) ; assert ! (! result . is_null () , "Result of bssl_sys::HMAC was null") ; out }
};
}
