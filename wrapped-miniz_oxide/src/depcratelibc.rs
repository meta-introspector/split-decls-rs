// Generated macro for libc (module)
macro_rules! Depcratelibc {
() => {
// Module: crate
// Provides: {"libc"}
// Dependencies: {}
# [cfg (any (feature = "libc_stub" , all (target_arch = "wasm32" , not (target_os = "emscripten"))))] mod libc { # ! [allow (non_camel_case_types)] use std :: alloc :: { alloc as rust_alloc , dealloc as rust_dealloc , realloc as rust_realloc , Layout , } ; use std :: mem ; pub type c_void = u8 ; pub type c_int = i32 ; pub type c_uint = u32 ; pub type c_ulong = u64 ; pub type c_char = i8 ; pub type size_t = usize ; pub unsafe fn malloc (a : size_t) -> * mut c_void { let size = a + mem :: size_of :: < size_t > () ; let layout = match Layout :: from_size_align (size , mem :: align_of :: < size_t > ()) { Ok (n) => n , Err (_) => return 0 as * mut c_void , } ; let ptr = rust_alloc (layout) as * mut size_t ; * ptr . offset (0) = size ; ptr . offset (1) as * mut c_void } pub unsafe fn realloc (ptr : * mut c_void , a : size_t) -> * mut c_void { let new_size = a + mem :: size_of :: < size_t > () ; let ptr = (ptr as * mut size_t) . offset (- 1) ; let old_size = * ptr . offset (0) ; let layout = Layout :: from_size_align_unchecked (old_size , mem :: size_of :: < size_t > ()) ; let ptr = rust_realloc (ptr as * mut _ , layout , new_size) as * mut size_t ; * ptr . offset (0) = new_size ; ptr . offset (1) as * mut c_void } pub unsafe fn free (ptr : * mut c_void) { let ptr = (ptr as * mut size_t) . offset (- 1) ; let size = * ptr . offset (0) ; let align = mem :: size_of :: < size_t > () ; let layout = Layout :: from_size_align_unchecked (size , align) ; rust_dealloc (ptr as * mut _ , layout) ; } }
};
}
