// Generated macro for RmpWriteErr (trait)
macro_rules! Depcrate_encodeRmpWriteErr {
() => {
// Module: crate::encode
// Provides: {"RmpWriteErr"}
// Dependencies: {}
# [doc = " The error type for operations on the [`RmpWrite`] trait."] # [doc = ""] # [doc = " For [`std::io::Write`], this is [`std::io::Error`]"] # [doc = " For [`ByteBuf`], this is [`core::convert::Infallible`]"] pub trait RmpWriteErr : Display + Debug + crate :: errors :: MaybeErrBound + 'static { }
};
}
