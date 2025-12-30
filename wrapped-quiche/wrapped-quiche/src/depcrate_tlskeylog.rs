// Generated macro for keylog (function)
macro_rules! Depcrate_tlskeylog {
() => {
// Module: crate::tls
// Provides: {"keylog"}
// Dependencies: {}
extern "C" fn keylog (ssl : * const SSL , line : * const c_char) { let ex_data = match ExData :: from_ssl_ptr (ssl) { Some (v) => v , None => return , } ; if let Some (keylog) = & mut ex_data . keylog { let data = unsafe { ffi :: CStr :: from_ptr (line) . to_bytes () } ; let mut full_line = Vec :: with_capacity (data . len () + 1) ; full_line . extend_from_slice (data) ; full_line . push (b'\n') ; keylog . write_all (& full_line [..]) . ok () ; keylog . flush () . ok () ; } }
};
}
