// Generated macro for NSString (trait)
macro_rules! Depcrate_foundationNSString {
() => {
// Module: crate::foundation
// Provides: {"NSString"}
// Dependencies: {}
pub trait NSString : Sized { unsafe fn alloc (_ : Self) -> id { msg_send ! [class ! (NSString) , alloc] } unsafe fn stringByAppendingString_ (self , other : id) -> id ; unsafe fn init_str (self , string : & str) -> Self ; unsafe fn UTF8String (self) -> * const c_char ; unsafe fn len (self) -> usize ; unsafe fn isEqualToString (self , string : & str) -> bool ; unsafe fn substringWithRange (self , range : NSRange) -> id ; }
};
}
