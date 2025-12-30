// Generated macro for impl_862 (impl)
macro_rules! Depcrate_output_fmtimpl_862 {
() => {
// Module: crate::output::fmt
// Provides: {"impl_862"}
// Dependencies: {}
impl Colorizer { pub (crate) fn new (stream : Stream , color_when : ColorChoice) -> Self { Colorizer { stream , color_when , content : Default :: default () , } } pub (crate) fn with_content (mut self , content : StyledStr) -> Self { self . content = content ; self } }
};
}
