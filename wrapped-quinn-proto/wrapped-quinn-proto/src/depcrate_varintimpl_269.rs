// Generated macro for impl_269 (impl)
macro_rules! Depcrate_varintimpl_269 {
() => {
// Module: crate::varint
// Provides: {"impl_269"}
// Dependencies: {}
impl std :: convert :: TryFrom < u64 > for VarInt { type Error = VarIntBoundsExceeded ; # [doc = " Succeeds iff `x` < 2^62"] fn try_from (x : u64) -> Result < Self , VarIntBoundsExceeded > { Self :: from_u64 (x) } }
};
}
