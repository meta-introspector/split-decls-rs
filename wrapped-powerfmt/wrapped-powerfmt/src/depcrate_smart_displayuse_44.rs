// Generated macro for use_44 (pub_use)
macro_rules! Depcrate_smart_displayuse_44 {
() => {
// Module: crate::smart_display
// Provides: {"use_44"}
// Dependencies: {}
# [doc = " Declare a private metadata type for `SmartDisplay`."] # [doc = ""] # [doc = " Use this attribute if you want to provide metadata for a type that is not public. Doing"] # [doc = " this will avoid a compiler error about a private type being used publicly. Keep in mind"] # [doc = " that any public fields, public methods, and trait implementations _will_ be able to be used"] # [doc = " by downstream users."] # [doc = ""] # [doc = " To avoid accidentally exposing details, such as when all fields are public or if the type"] # [doc = " is a unit struct, the type is annotated with `#[non_exhaustive]` automatically."] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " # use powerfmt::smart_display;"] # [doc = " /// Metadata for `Foo`"] # [doc = " #[smart_display::private_metadata]"] # [doc = " #[derive(Debug)]"] # [doc = " pub(crate) struct FooMetadata {"] # [doc = "     pub(crate) expensive_to_calculate: usize,"] # [doc = " }"] # [doc = " ```"] # [cfg (feature = "macros")] pub use powerfmt_macros :: smart_display_private_metadata as private_metadata ;
};
}
