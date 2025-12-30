// Generated macro for impl_11 (impl)
macro_rules! Depcrate_idimpl_11 {
() => {
// Module: crate::id
// Provides: {"impl_11"}
// Dependencies: {}
impl Id { # [doc = " Returns the CAN Identifier as a raw 32-bit integer, ignoring the distinction between"] # [doc = " standard ID and extended ID."] # [doc = ""] # [doc = " This function ignores that a standard ID and an extended ID are different IDs, even"] # [doc = " if their numerical values are the same. It should only be used if the raw numerical value"] # [doc = " is required and the distinction between standard ID and extended ID is irrelevant."] # [doc = ""] # [doc = " In all other cases, it is recommended to de-structure the ID with a match statement."] # [inline] pub const fn as_raw_unchecked (& self) -> u32 { match self { Id :: Standard (id) => id . as_raw () as u32 , Id :: Extended (id) => id . as_raw () , } } }
};
}
