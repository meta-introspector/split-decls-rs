// Generated macro for EncodeArguments (trait)
macro_rules! Depcrate_encodeEncodeArguments {
() => {
// Module: crate::encode
// Provides: {"EncodeArguments"}
// Dependencies: {}
# [doc = " Types that represent an ordered group of function arguments, where each"] # [doc = " argument has an Objective-C type-encoding, or can be converted from one."] # [doc = ""] # [doc = " This is implemented for tuples of up to 16 arguments, where each argument"] # [doc = " implements [`EncodeArgument`]. It is a sealed trait, and should not need"] # [doc = " to be implemented manually - it is primarily used to make generic code a"] # [doc = " bit easier to read and understand."] # [doc = ""] # [doc = " Note that tuples themselves don't implement [`Encode`] directly, because"] # [doc = " they're not FFI-safe!"] pub trait EncodeArguments : args_private :: Sealed { # [doc = " The encodings for the arguments."] const ENCODINGS : & 'static [Encoding] ; # [doc = " Invoke a message sending function with the given object, selector,"] # [doc = " and arguments."] # [doc = ""] # [doc = " Implementation-wise, this is a bit ugly, but simply just easiest to"] # [doc = " have the method on this trait, since inside `MessageReceiver` we only"] # [doc = " want to publicly require `EncodeArguments`, and not another private"] # [doc = " trait."] # [doc (hidden)] unsafe fn __invoke < R : EncodeReturn > (msg_send_fn : Imp , receiver : * mut AnyObject , sel : Sel , args : Self ,) -> R ; }
};
}
