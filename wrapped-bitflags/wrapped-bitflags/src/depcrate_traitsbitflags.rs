// Generated macro for BitFlags (trait)
macro_rules! Depcrate_traitsBitFlags {
() => {
// Module: crate::traits
// Provides: {"BitFlags"}
// Dependencies: {}
# [doc (hidden)] # [deprecated (note = "use the `Flags` trait instead")] pub trait BitFlags : ImplementedByBitFlagsMacro + Flags { # [doc = " An iterator over enabled flags in an instance of the type."] type Iter : Iterator < Item = Self > ; # [doc = " An iterator over the raw names and bits for enabled flags in an instance of the type."] type IterNames : Iterator < Item = (& 'static str , Self) > ; }
};
}
