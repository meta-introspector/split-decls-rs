// Generated macro for Filter (struct)
macro_rules! Depcrate_imageops_sampleFilter {
() => {
// Module: crate::imageops::sample
// Provides: {"Filter"}
// Dependencies: {}
# [doc = " A Representation of a separable filter."] pub (crate) struct Filter < 'a > { # [doc = " The filter's filter function."] pub (crate) kernel : Box < dyn Fn (f32) -> f32 + 'a > , # [doc = " The window on which this filter operates."] pub (crate) support : f32 , }
};
}
