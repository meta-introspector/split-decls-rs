// Generated macro for impl_61 (impl)
macro_rules! Depcrate_errorimpl_61 {
() => {
// Module: crate::error
// Provides: {"impl_61"}
// Dependencies: {}
impl < Item , Range , Position > ParseError < Item , Range , Position > for UnexpectedParse where Position : Default , { type StreamError = Self ; # [inline] fn empty (_position : Position) -> Self { UnexpectedParse :: Unexpected } # [inline] fn from_error (_ : Position , err : Self :: StreamError) -> Self { err } fn position (& self) -> Position { Position :: default () } # [inline] fn set_position (& mut self , _position : Position) { } # [inline] fn add (& mut self , err : Self :: StreamError) { * self = match (* self , err) { (UnexpectedParse :: Eoi , _) => UnexpectedParse :: Eoi , (_ , err) => err , } ; } # [inline] fn set_expected < F > (self_ : & mut Tracked < Self > , info : Self :: StreamError , f : F) where F : FnOnce (& mut Tracked < Self >) , { f (self_) ; self_ . error = info ; } fn is_unexpected_end_of_input (& self) -> bool { * self == UnexpectedParse :: Eoi } # [inline] fn into_other < T > (self) -> T where T : ParseError < Item , Range , Position > , { T :: from_error (Position :: default () , StreamError :: into_other (self)) } }
};
}
