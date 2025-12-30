// Generated macro for impl_1486 (impl)
macro_rules! Depcrate_write_utilimpl_1486 {
() => {
// Module: crate::write::util
// Provides: {"impl_1486"}
// Dependencies: {}
impl < 'a > BytesMut for & 'a mut [u8] { # [inline] fn write_at < T : Pod > (self , offset : usize , val : & T) -> Result < () , () > { let src = bytes_of (val) ; let dest = self . get_mut (offset ..) . ok_or (()) ? ; let dest = dest . get_mut (.. src . len ()) . ok_or (()) ? ; dest . copy_from_slice (src) ; Ok (()) } }
};
}
