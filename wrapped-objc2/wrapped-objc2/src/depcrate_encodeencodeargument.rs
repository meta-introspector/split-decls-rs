// Generated macro for EncodeArgument (trait)
macro_rules! Depcrate_encodeEncodeArgument {
() => {
// Module: crate::encode
// Provides: {"EncodeArgument"}
// Dependencies: {}
# [doc = " Types that are safe as arguments to Objective-C methods."] # [doc = ""] # [doc = " This is a sealed trait, and should not need to be implemented manually."] # [doc = ""] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Similar to [`Encode`], except the value is only guaranteed to be valid as"] # [doc = " an argument or a parameter, both from functions/methods you're calling and"] # [doc = " from declared functions/methods."] # [doc = ""] # [doc = " It does not have to be valid as e.g. an instance variable, or as an"] # [doc = " argument to a function."] pub unsafe trait EncodeArgument : argument_private :: Sealed { # [doc = " The Objective-C type-encoding for this type."] const ENCODING_ARGUMENT : Encoding ; }
};
}
