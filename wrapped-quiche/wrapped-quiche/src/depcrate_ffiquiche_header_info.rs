// Generated macro for quiche_header_info (function)
macro_rules! Depcrate_ffiquiche_header_info {
() => {
// Module: crate::ffi
// Provides: {"quiche_header_info"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_header_info (buf : * mut u8 , buf_len : size_t , dcil : size_t , version : * mut u32 , ty : * mut u8 , scid : * mut u8 , scid_len : * mut size_t , dcid : * mut u8 , dcid_len : * mut size_t , token : * mut u8 , token_len : * mut size_t ,) -> c_int { let buf = unsafe { slice :: from_raw_parts_mut (buf , buf_len) } ; let hdr = match Header :: from_slice (buf , dcil) { Ok (v) => v , Err (e) => return e . to_c () as c_int , } ; unsafe { * version = hdr . version ; * ty = match hdr . ty { Type :: Initial => 1 , Type :: Retry => 2 , Type :: Handshake => 3 , Type :: ZeroRTT => 4 , Type :: Short => 5 , Type :: VersionNegotiation => 6 , } ; if * scid_len < hdr . scid . len () { return - 1 ; } let scid = slice :: from_raw_parts_mut (scid , * scid_len) ; let scid = & mut scid [.. hdr . scid . len ()] ; scid . copy_from_slice (& hdr . scid) ; * scid_len = hdr . scid . len () ; if * dcid_len < hdr . dcid . len () { return - 1 ; } let dcid = slice :: from_raw_parts_mut (dcid , * dcid_len) ; let dcid = & mut dcid [.. hdr . dcid . len ()] ; dcid . copy_from_slice (& hdr . dcid) ; * dcid_len = hdr . dcid . len () ; match hdr . token { Some (tok) => { if * token_len < tok . len () { return - 1 ; } let token = slice :: from_raw_parts_mut (token , * token_len) ; let token = & mut token [.. tok . len ()] ; token . copy_from_slice (& tok) ; * token_len = tok . len () ; } , None => * token_len = 0 , } } 0 }
};
}
