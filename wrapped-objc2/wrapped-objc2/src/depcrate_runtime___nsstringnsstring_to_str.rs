// Generated macro for nsstring_to_str (function)
macro_rules! Depcrate_runtime___nsstringnsstring_to_str {
() => {
// Module: crate::runtime::__nsstring
// Provides: {"nsstring_to_str"}
// Dependencies: {}
# [doc = " Extract a [`str`](`prim@str`) representation out of the given NSString."] # [doc = ""] # [doc = " Uses [`UTF8String`] under the hood."] # [doc = ""] # [doc = " [`UTF8String`]: https://developer.apple.com/documentation/foundation/nsstring/1411189-utf8string?language=objc"] # [doc = ""] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - The object must be an instance of `NSString`."] # [doc = " - The returned string must not be moved outside the autorelease pool into"] # [doc = "   which it (may) have been released."] # [doc = ""] # [doc = " Furthermore, the object must not, as is always the case for strings, be"] # [doc = " mutated in parallel."] pub unsafe fn nsstring_to_str < 'r , 's : 'r , 'p : 'r > (obj : & 's NSObject , pool : AutoreleasePool < 'p > ,) -> & 'r str { pool . __verify_is_inner () ; let bytes : * const c_char = unsafe { msg_send ! [obj , UTF8String] } ; let bytes : * const u8 = bytes . cast () ; let len = unsafe { nsstring_len (obj) } ; let bytes : & 'r [u8] = unsafe { slice :: from_raw_parts (bytes , len) } ; # [cfg (not (debug_assertions))] unsafe { str :: from_utf8_unchecked (bytes) } # [cfg (debug_assertions)] { str :: from_utf8 (bytes) . expect ("invalid UTF-8 in NSString") } }
};
}
