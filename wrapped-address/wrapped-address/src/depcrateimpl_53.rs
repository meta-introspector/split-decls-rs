// Generated macro for impl_53 (impl)
macro_rules! Depcrateimpl_53 {
() => {
// Module: crate
// Provides: {"impl_53"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl TryFrom < Vec < u8 > > for Address { type Error = Vec < u8 > ; # [inline] fn try_from (address : Vec < u8 >) -> Result < Self , Self :: Error > { < [u8 ; 32] > :: try_from (address) . map (Self :: from) } }
};
}
