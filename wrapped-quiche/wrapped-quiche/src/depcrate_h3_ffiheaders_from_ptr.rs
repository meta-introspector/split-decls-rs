// Generated macro for headers_from_ptr (function)
macro_rules! Depcrate_h3_ffiheaders_from_ptr {
() => {
// Module: crate::h3::ffi
// Provides: {"headers_from_ptr"}
// Dependencies: {}
fn headers_from_ptr < 'a > (ptr : * const Header , len : size_t ,) -> Vec < h3 :: HeaderRef < 'a > > { let headers = unsafe { slice :: from_raw_parts (ptr , len) } ; let mut out = Vec :: new () ; for h in headers { out . push ({ let name = unsafe { slice :: from_raw_parts (h . name , h . name_len) } ; let value = unsafe { slice :: from_raw_parts (h . value , h . value_len) } ; h3 :: HeaderRef :: new (name , value) }) ; } out }
};
}
