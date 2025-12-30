// Generated macro for impl_52 (impl)
macro_rules! Depcrateimpl_52 {
() => {
// Module: crate
// Provides: {"impl_52"}
// Dependencies: {}
impl TryFrom < & [u8] > for Address { type Error = array :: TryFromSliceError ; # [inline] fn try_from (address : & [u8]) -> Result < Self , Self :: Error > { < [u8 ; 32] > :: try_from (address) . map (Self :: from) } }
};
}
