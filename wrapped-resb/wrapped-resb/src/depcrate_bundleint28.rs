// Generated macro for Int28 (struct)
macro_rules! Depcrate_bundleInt28 {
() => {
// Module: crate::bundle
// Provides: {"Int28"}
// Dependencies: {}
# [doc = " A 28-bit integer of undetermined signedness."] # [doc = ""] # [doc = " [`Resource`]s may include 28-bit integers whose signedness is determined at"] # [doc = " runtime by consumers. Because these integers are stored in a 32-bit value,"] # [doc = " negative values in signed integers require special handling, provided by"] # [doc = " this newtype wrapper."] # [derive (Copy , Clone , Debug , PartialEq)] pub struct Int28 (u32) ;
};
}
