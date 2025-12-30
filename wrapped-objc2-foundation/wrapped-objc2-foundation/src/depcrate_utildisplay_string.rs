// Generated macro for display_string (function)
macro_rules! Depcrate_utildisplay_string {
() => {
// Module: crate::util
// Provides: {"display_string"}
// Dependencies: {}
# [doc = " Display the string."] # [doc = ""] # [doc = " Put here to allow using it without the `\"NSString\"` feature being active."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The string must be an instance of `NSString`."] pub (crate) unsafe fn display_string (string : & NSObject , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { autoreleasepool_leaking (| pool | fmt :: Display :: fmt (unsafe { nsstring_to_str (string , pool) } , f)) }
};
}
