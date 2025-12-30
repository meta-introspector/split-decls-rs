// Generated macro for select_alpn (function)
macro_rules! Depcrate_tlsselect_alpn {
() => {
// Module: crate::tls
// Provides: {"select_alpn"}
// Dependencies: {}
extern "C" fn select_alpn (ssl : * mut SSL , out : * mut * const u8 , out_len : * mut u8 , inp : * mut u8 , in_len : c_uint , _arg : * mut c_void ,) -> c_int { let ex_data = match ExData :: from_ssl_ptr (ssl) { Some (v) => v , None => return TLS_ERROR , } ; if ex_data . application_protos . is_empty () { return TLS_ERROR ; } let mut protos = octets :: Octets :: with_slice (unsafe { slice :: from_raw_parts (inp , in_len as usize) }) ; while let Ok (proto) = protos . get_bytes_with_u8_length () { let found = ex_data . application_protos . iter () . any (| expected | { trace ! ("checking peer ALPN {:?} against {:?}" , std :: str :: from_utf8 (proto . as_ref ()) , std :: str :: from_utf8 (expected . as_slice ())) ; if expected . len () == proto . len () && expected . as_slice () == proto . as_ref () { unsafe { * out = expected . as_slice () . as_ptr () ; * out_len = expected . len () as u8 ; } return true ; } false }) ; if found { return 0 ; } } TLS_ERROR }
};
}
