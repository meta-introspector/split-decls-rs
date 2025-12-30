// Generated macro for impl_731 (impl)
macro_rules! Depcrate_output_render_sizeimpl_731 {
() => {
// Module: crate::output::render::size
// Provides: {"impl_731"}
// Dependencies: {}
impl f :: DeviceIDs { fn render < C : Colours > (self , colours : & C) -> TextCell { let major = self . major . to_string () ; let minor = self . minor . to_string () ; TextCell { width : DisplayWidth :: from (major . len () + 1 + minor . len ()) , contents : vec ! [colours . major () . paint (major) , colours . comma () . paint (",") , colours . minor () . paint (minor) ,] . into () , } } }
};
}
