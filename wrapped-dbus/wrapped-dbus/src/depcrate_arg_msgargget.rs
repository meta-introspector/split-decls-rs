// Generated macro for Get (trait)
macro_rules! Depcrate_arg_msgargGet {
() => {
// Module: crate::arg::msgarg
// Provides: {"Get"}
// Dependencies: {}
# [doc = " Types that can be retrieved from a message as arguments implement this trait."] pub trait Get < 'a > : Sized { # [doc = " Performs the get operation."] fn get (i : & mut Iter < 'a >) -> Option < Self > ; }
};
}
