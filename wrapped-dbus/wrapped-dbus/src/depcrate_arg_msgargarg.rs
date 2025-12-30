// Generated macro for Arg (trait)
macro_rules! Depcrate_arg_msgargArg {
() => {
// Module: crate::arg::msgarg
// Provides: {"Arg"}
// Dependencies: {}
# [doc = " Types that can represent a D-Bus message argument implement this trait."] # [doc = ""] # [doc = " Types should also implement either Append or Get to be useful."] pub trait Arg { # [doc = " The corresponding D-Bus argument type code."] const ARG_TYPE : ArgType ; # [doc = " The corresponding D-Bus type signature for this type."] fn signature () -> Signature < 'static > ; }
};
}
