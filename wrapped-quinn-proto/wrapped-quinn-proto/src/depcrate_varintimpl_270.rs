// Generated macro for impl_270 (impl)
macro_rules! Depcrate_varintimpl_270 {
() => {
// Module: crate::varint
// Provides: {"impl_270"}
// Dependencies: {}
impl std :: convert :: TryFrom < u128 > for VarInt { type Error = VarIntBoundsExceeded ; # [doc = " Succeeds iff `x` < 2^62"] fn try_from (x : u128) -> Result < Self , VarIntBoundsExceeded > { Self :: from_u64 (x . try_into () . map_err (| _ | VarIntBoundsExceeded) ?) } }
};
}
