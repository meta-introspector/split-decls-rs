// Generated macro for impl_129 (impl)
macro_rules! Depcrate_entry_modeimpl_129 {
() => {
// Module: crate::entry::mode
// Provides: {"impl_129"}
// Dependencies: {}
impl From < gix_object :: tree :: EntryMode > for Mode { fn from (value : gix_object :: tree :: EntryMode) -> Self { let value : u16 = value . value () ; Self :: from_bits_truncate (u32 :: from (value)) } }
};
}
