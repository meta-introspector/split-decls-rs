// Generated macro for TokioAdapter (struct)
macro_rules! Depcrate_reader_async_tokioTokioAdapter {
() => {
// Module: crate::reader::async_tokio
// Provides: {"TokioAdapter"}
// Dependencies: {}
# [doc = " A struct for read XML asynchronously from an [`AsyncBufRead`]."] # [doc = ""] # [doc = " Having own struct allows us to implement anything without risk of name conflicts"] # [doc = " and does not suffer from the impossibility of having `async` in traits."] struct TokioAdapter < 'a , R > (& 'a mut R) ;
};
}
