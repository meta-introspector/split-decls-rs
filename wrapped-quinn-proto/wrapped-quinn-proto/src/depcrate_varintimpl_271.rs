// Generated macro for impl_271 (impl)
macro_rules! Depcrate_varintimpl_271 {
() => {
// Module: crate::varint
// Provides: {"impl_271"}
// Dependencies: {}
impl std :: convert :: TryFrom < usize > for VarInt { type Error = VarIntBoundsExceeded ; # [doc = " Succeeds iff `x` < 2^62"] fn try_from (x : usize) -> Result < Self , VarIntBoundsExceeded > { Self :: try_from (x as u64) } }
};
}
