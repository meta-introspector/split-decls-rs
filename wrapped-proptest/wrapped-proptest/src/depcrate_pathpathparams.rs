// Generated macro for PathParams (struct)
macro_rules! Depcrate_pathPathParams {
() => {
// Module: crate::path
// Provides: {"PathParams"}
// Dependencies: {}
# [doc = " Parameters for the [`Arbitrary`] implementation for [`PathBuf`]."] # [doc = ""] # [doc = " By default, this generates paths with 0 to 8 components uniformly at random, each of which is a"] # [doc = " default [`StringParam`]."] # [derive (Clone , Debug , PartialEq , Eq , Hash)] pub struct PathParams { # [doc = " The number of components in the path."] components : SizeRange , # [doc = " The regular expression to generate individual components."] component_regex : StringParam , }
};
}
