// Generated macro for impl_44 (impl)
macro_rules! Depcrate_foundationimpl_44 {
() => {
// Module: crate::foundation
// Provides: {"impl_44"}
// Dependencies: {}
impl NSString for id { unsafe fn isEqualToString (self , other : & str) -> bool { let other = NSString :: alloc (nil) . init_str (other) ; let rv : BOOL = msg_send ! [self , isEqualToString : other] ; let _ : () = msg_send ! [other , release] ; rv != NO } unsafe fn stringByAppendingString_ (self , other : id) -> id { msg_send ! [self , stringByAppendingString : other] } unsafe fn init_str (self , string : & str) -> id { msg_send ! [self , initWithBytes : string . as_ptr () as * const c_void length : string . len () encoding : UTF8_ENCODING] } unsafe fn len (self) -> usize { msg_send ! [self , lengthOfBytesUsingEncoding : UTF8_ENCODING] } unsafe fn UTF8String (self) -> * const c_char { msg_send ! [self , UTF8String] } unsafe fn substringWithRange (self , range : NSRange) -> id { msg_send ! [self , substringWithRange : range] } }
};
}
