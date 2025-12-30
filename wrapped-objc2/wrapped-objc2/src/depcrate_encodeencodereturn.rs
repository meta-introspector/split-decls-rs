// Generated macro for EncodeReturn (trait)
macro_rules! Depcrate_encodeEncodeReturn {
() => {
// Module: crate::encode
// Provides: {"EncodeReturn"}
// Dependencies: {}
# [doc = " Types that are safe as the return value from Objective-C."] # [doc = ""] # [doc = " This is a sealed trait, and should not need to be implemented manually."] # [doc = ""] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Similar to [`Encode`], except the value is only guaranteed to be valid as"] # [doc = " a return value, both from functions/methods you're calling, and from"] # [doc = " declared functions/methods."] # [doc = ""] # [doc = " It does not have to be valid as e.g. an instance variable, or as an"] # [doc = " argument to a function."] pub unsafe trait EncodeReturn : return_private :: Sealed { # [doc = " The Objective-C type-encoding for this type."] const ENCODING_RETURN : Encoding ; }
};
}
