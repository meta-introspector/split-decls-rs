// Generated macro for impl_168 (impl)
macro_rules! Depcrate_stream_easyimpl_168 {
() => {
// Module: crate::stream::easy
// Provides: {"impl_168"}
// Dependencies: {}
impl < Item , Range , Position > crate :: error :: ParseError < Item , Range , Position > for Error < Item , Range > where Item : PartialEq , Range : PartialEq , Position : Default , { type StreamError = Self ; # [inline] fn empty (_ : Position) -> Self { Self :: message_static_message ("") } # [inline] fn from_error (_ : Position , err : Self :: StreamError) -> Self { err } # [inline] fn position (& self) -> Position { Position :: default () } # [inline] fn set_position (& mut self , _position : Position) { } # [inline] fn add (& mut self , err : Self :: StreamError) { * self = err ; } # [inline] fn set_expected < F > (self_ : & mut Tracked < Self > , info : Self :: StreamError , f : F) where F : FnOnce (& mut Tracked < Self >) , { f (self_) ; self_ . error = info ; } fn is_unexpected_end_of_input (& self) -> bool { * self == Self :: end_of_input () } # [inline] fn into_other < T > (self) -> T where T : crate :: error :: ParseError < Item , Range , Position > , { T :: from_error (Position :: default () , StreamError :: into_other (self)) } }
};
}
