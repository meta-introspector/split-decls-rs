// Generated macro for impl_227 (impl)
macro_rules! Depcrate_stream_readimpl_227 {
() => {
// Module: crate::stream::read
// Provides: {"impl_227"}
// Dependencies: {}
impl < Item , Range , Position > ParseError < Item , Range , Position > for Error where Position : Default , { type StreamError = Self ; # [inline] fn empty (_position : Position) -> Self { Error :: Unexpected } # [inline] fn from_error (_ : Position , err : Self :: StreamError) -> Self { err } # [inline] fn set_position (& mut self , _position : Position) { } # [inline] fn add (& mut self , err : Self :: StreamError) { * self = match (& * self , err) { (Error :: EndOfInput , _) => Error :: EndOfInput , (_ , err) => err , } ; } # [inline] fn set_expected < F > (self_ : & mut Tracked < Self > , info : Self :: StreamError , f : F) where F : FnOnce (& mut Tracked < Self >) , { f (self_) ; self_ . error = info ; } fn is_unexpected_end_of_input (& self) -> bool { * self == Error :: EndOfInput } # [inline] fn into_other < T > (self) -> T where T : ParseError < Item , Range , Position > , { T :: from_error (Position :: default () , StreamError :: into_other (self)) } }
};
}
