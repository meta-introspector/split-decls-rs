// Generated macro for nsstring_len (function)
macro_rules! Depcrate_runtime___nsstringnsstring_len {
() => {
// Module: crate::runtime::__nsstring
// Provides: {"nsstring_len"}
// Dependencies: {}
# [doc = " The number of UTF-8 code units in the given string."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The object must be an instance of `NSString`."] # [inline] pub unsafe fn nsstring_len (obj : & NSObject) -> NSUInteger { unsafe { msg_send ! [obj , lengthOfBytesUsingEncoding : UTF8_ENCODING] } }
};
}
