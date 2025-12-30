// Generated macro for impl_1353 (impl)
macro_rules! Depcrate_sliceimpl_1353 {
() => {
// Module: crate::slice
// Provides: {"impl_1353"}
// Dependencies: {}
impl [u8] { # [doc = " Returns a vector containing a copy of this slice where each byte"] # [doc = " is mapped to its ASCII upper case equivalent."] # [doc = ""] # [doc = " ASCII letters 'a' to 'z' are mapped to 'A' to 'Z',"] # [doc = " but non-ASCII letters are unchanged."] # [doc = ""] # [doc = " To uppercase the value in-place, use [`make_ascii_uppercase`]."] # [doc = ""] # [doc = " [`make_ascii_uppercase`]: slice::make_ascii_uppercase"] # [cfg (not (no_global_oom_handling))] # [rustc_allow_incoherent_impl] # [must_use = "this returns the uppercase bytes as a new Vec, \
                  without modifying the original"] # [stable (feature = "ascii_methods_on_intrinsics" , since = "1.23.0")] # [inline] pub fn to_ascii_uppercase (& self) -> Vec < u8 > { let mut me = self . to_vec () ; me . make_ascii_uppercase () ; me } # [doc = " Returns a vector containing a copy of this slice where each byte"] # [doc = " is mapped to its ASCII lower case equivalent."] # [doc = ""] # [doc = " ASCII letters 'A' to 'Z' are mapped to 'a' to 'z',"] # [doc = " but non-ASCII letters are unchanged."] # [doc = ""] # [doc = " To lowercase the value in-place, use [`make_ascii_lowercase`]."] # [doc = ""] # [doc = " [`make_ascii_lowercase`]: slice::make_ascii_lowercase"] # [cfg (not (no_global_oom_handling))] # [rustc_allow_incoherent_impl] # [must_use = "this returns the lowercase bytes as a new Vec, \
                  without modifying the original"] # [stable (feature = "ascii_methods_on_intrinsics" , since = "1.23.0")] # [inline] pub fn to_ascii_lowercase (& self) -> Vec < u8 > { let mut me = self . to_vec () ; me . make_ascii_lowercase () ; me } }
};
}
