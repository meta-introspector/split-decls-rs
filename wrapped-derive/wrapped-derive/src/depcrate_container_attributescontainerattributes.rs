// Generated macro for ContainerAttributes (struct)
macro_rules! Depcrate_container_attributesContainerAttributes {
() => {
// Module: crate::container_attributes
// Provides: {"ContainerAttributes"}
// Dependencies: {}
pub struct ContainerAttributes { # [doc = " Specify type bounds to be applied to the derived `Arbitrary` implementation instead of the"] # [doc = " default inferred bounds."] # [doc = ""] # [doc = " ```ignore"] # [doc = " #[arbitrary(bound = \"T: Default, U: Debug\")]"] # [doc = " ```"] # [doc = ""] # [doc = " Multiple attributes will be combined as long as they don't conflict, e.g."] # [doc = ""] # [doc = " ```ignore"] # [doc = " #[arbitrary(bound = \"T: Default\")]"] # [doc = " #[arbitrary(bound = \"U: Default\")]"] # [doc = " ```"] pub bounds : Option < Vec < Punctuated < TypeParam , Token ! [,] > > > , }
};
}
