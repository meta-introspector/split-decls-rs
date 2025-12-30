// Generated macro for load_next (function)
macro_rules! Depcrateload_next {
() => {
// Module: crate
// Provides: {"load_next"}
// Dependencies: {}
fn load_next < Prototype : Copy > (name : & [u8]) -> Prototype { unsafe { let name = CStr :: from_bytes_with_nul (name) . expect ("invalid c-string literal") ; let sym = dlsym (RTLD_NEXT , name . as_ptr ()) ; if sym . is_null () { error ! ("failed to load libc function {:?}" , name . to_string_lossy ()) ; panic ! ("failed to load libc function pointer") ; } * (& sym as * const * mut c_void as * const Prototype) } }
};
}
