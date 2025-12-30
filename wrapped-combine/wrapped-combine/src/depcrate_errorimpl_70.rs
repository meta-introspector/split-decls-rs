// Generated macro for impl_70 (impl)
macro_rules! Depcrate_errorimpl_70 {
() => {
// Module: crate::error
// Provides: {"impl_70"}
// Dependencies: {}
impl < Item , Range , Position > ParseError < Item , Range , Position > for StringStreamError where Position : Default , { type StreamError = Self ; # [inline] fn empty (_position : Position) -> Self { StringStreamError :: UnexpectedParse } # [inline] fn from_error (_ : Position , err : Self :: StreamError) -> Self { err } fn position (& self) -> Position { Position :: default () } # [inline] fn set_position (& mut self , _position : Position) { } # [inline] fn add (& mut self , err : Self :: StreamError) { * self = match (* self , err) { (StringStreamError :: Eoi , _) => StringStreamError :: Eoi , (_ , err) => err , } ; } # [inline] fn set_expected < F > (self_ : & mut Tracked < Self > , info : Self :: StreamError , f : F) where F : FnOnce (& mut Tracked < Self >) , { f (self_) ; self_ . error = info ; } fn is_unexpected_end_of_input (& self) -> bool { * self == StringStreamError :: Eoi } # [inline] fn into_other < T > (self) -> T where T : ParseError < Item , Range , Position > , { T :: from_error (Position :: default () , StreamError :: into_other (self)) } }
};
}
