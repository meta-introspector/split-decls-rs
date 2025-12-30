// Generated macro for header_cb (function)
macro_rules! Depcrate_easy_handlerheader_cb {
() => {
// Module: crate::easy::handler
// Provides: {"header_cb"}
// Dependencies: {}
extern "C" fn header_cb < H : Handler > (buffer : * mut c_char , size : size_t , nitems : size_t , userptr : * mut c_void ,) -> size_t { let keep_going = panic :: catch (| | unsafe { let data = slice :: from_raw_parts (buffer as * const u8 , size * nitems) ; (* (userptr as * mut Inner < H >)) . handler . header (data) }) . unwrap_or (false) ; if keep_going { size * nitems } else { ! 0 } }
};
}
