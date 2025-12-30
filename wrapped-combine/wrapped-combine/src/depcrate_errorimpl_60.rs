// Generated macro for impl_60 (impl)
macro_rules! Depcrate_errorimpl_60 {
() => {
// Module: crate::error
// Provides: {"impl_60"}
// Dependencies: {}
impl < Item , Range > StreamError < Item , Range > for UnexpectedParse { # [inline] fn unexpected_token (_ : Item) -> Self { UnexpectedParse :: Unexpected } # [inline] fn unexpected_range (_ : Range) -> Self { UnexpectedParse :: Unexpected } # [inline] fn unexpected_format < T > (_ : T) -> Self where T : fmt :: Display , { UnexpectedParse :: Unexpected } # [inline] fn expected_token (_ : Item) -> Self { UnexpectedParse :: Unexpected } # [inline] fn expected_range (_ : Range) -> Self { UnexpectedParse :: Unexpected } # [inline] fn expected_format < T > (_ : T) -> Self where T : fmt :: Display , { UnexpectedParse :: Unexpected } # [inline] fn message_format < T > (_ : T) -> Self where T : fmt :: Display , { UnexpectedParse :: Unexpected } # [inline] fn message_token (_ : Item) -> Self { UnexpectedParse :: Unexpected } # [inline] fn message_range (_ : Range) -> Self { UnexpectedParse :: Unexpected } # [inline] fn end_of_input () -> Self { UnexpectedParse :: Eoi } # [inline] fn is_unexpected_end_of_input (& self) -> bool { * self == UnexpectedParse :: Eoi } # [inline] fn into_other < T > (self) -> T where T : StreamError < Item , Range > , { match self { UnexpectedParse :: Unexpected => T :: unexpected_static_message ("parse") , UnexpectedParse :: Eoi => T :: end_of_input () , } } }
};
}
