// Generated macro for impl_226 (impl)
macro_rules! Depcrate_stream_readimpl_226 {
() => {
// Module: crate::stream::read
// Provides: {"impl_226"}
// Dependencies: {}
impl < Item , Range > StreamError < Item , Range > for Error { # [inline] fn unexpected_token (_ : Item) -> Self { Error :: Unexpected } # [inline] fn unexpected_range (_ : Range) -> Self { Error :: Unexpected } # [inline] fn unexpected_format < T > (_ : T) -> Self where T : fmt :: Display , { Error :: Unexpected } # [inline] fn expected_token (_ : Item) -> Self { Error :: Unexpected } # [inline] fn expected_range (_ : Range) -> Self { Error :: Unexpected } # [inline] fn expected_format < T > (_ : T) -> Self where T : fmt :: Display , { Error :: Unexpected } # [inline] fn message_format < T > (_ : T) -> Self where T : fmt :: Display , { Error :: Unexpected } # [inline] fn message_token (_ : Item) -> Self { Error :: Unexpected } # [inline] fn message_range (_ : Range) -> Self { Error :: Unexpected } # [inline] fn end_of_input () -> Self { Error :: EndOfInput } # [inline] fn is_unexpected_end_of_input (& self) -> bool { * self == Error :: EndOfInput } # [inline] fn into_other < T > (self) -> T where T : StreamError < Item , Range > , { match self { Error :: Unexpected => T :: unexpected_static_message ("parse") , Error :: EndOfInput => T :: end_of_input () , Error :: Io (err) => T :: other (err) , } } }
};
}
