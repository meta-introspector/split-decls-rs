// Generated macro for create_null_error (function)
macro_rules! Depcrate___macros_msg_send_null_errorcreate_null_error {
() => {
// Module: crate::__macros::msg_send::null_error
// Provides: {"create_null_error"}
// Dependencies: {}
# [cold] fn create_null_error () -> NSErrorWrapper { autoreleasepool (| _ | { let cls = unsafe { CStr :: from_bytes_with_nul_unchecked (b"NSString\0") } ; let cls = AnyClass :: get (cls) . unwrap_or_else (foundation_not_linked) ; let domain = unsafe { CStr :: from_bytes_with_nul_unchecked (b"__objc2.missingError\0") } ; let domain : Retained < NSObject > = unsafe { msg_send ! [cls , stringWithUTF8String : domain . as_ptr ()] } ; let cls = unsafe { CStr :: from_bytes_with_nul_unchecked (b"NSError\0") } ; let cls = AnyClass :: get (cls) . unwrap_or_else (foundation_not_linked) ; let domain : & NSObject = & domain ; let code : NSInteger = 0 ; let user_info : Option < & NSObject > = None ; let err : Retained < NSObject > = unsafe { msg_send ! [cls , errorWithDomain : domain , code : code , userInfo : user_info] } ; NSErrorWrapper (err) }) }
};
}
