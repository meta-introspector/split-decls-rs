// Generated macro for impl_28 (impl)
macro_rules! Depcrate_stateimpl_28 {
() => {
// Module: crate::state
// Provides: {"impl_28"}
// Dependencies: {}
impl UpgradeableLoaderState { # [doc = " Size of a serialized program account."] pub const fn size_of_uninitialized () -> usize { 4 } # [doc = " Size of a buffer account's serialized metadata."] pub const fn size_of_buffer_metadata () -> usize { 37 } # [doc = " Size of a programdata account's serialized metadata."] pub const fn size_of_programdata_metadata () -> usize { 45 } # [doc = " Size of a serialized program account."] pub const fn size_of_program () -> usize { 36 } # [doc = " Size of a serialized buffer account."] pub const fn size_of_buffer (program_len : usize) -> usize { Self :: size_of_buffer_metadata () . saturating_add (program_len) } # [doc = " Size of a serialized programdata account."] pub const fn size_of_programdata (program_len : usize) -> usize { Self :: size_of_programdata_metadata () . saturating_add (program_len) } }
};
}
