// Generated macro for impl_482 (impl)
macro_rules! Depcrate_uri_schemeimpl_482 {
() => {
// Module: crate::uri::scheme
// Provides: {"impl_482"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a [u8] > for Scheme { type Error = InvalidUri ; # [inline] fn try_from (s : & 'a [u8]) -> Result < Self , Self :: Error > { use self :: Scheme2 :: * ; match Scheme2 :: parse_exact (s) ? { None => Err (ErrorKind :: InvalidScheme . into ()) , Standard (p) => Ok (Standard (p) . into ()) , Other (_) => { let bytes = Bytes :: copy_from_slice (s) ; let string = unsafe { ByteStr :: from_utf8_unchecked (bytes) } ; Ok (Other (Box :: new (string)) . into ()) } } } }
};
}
