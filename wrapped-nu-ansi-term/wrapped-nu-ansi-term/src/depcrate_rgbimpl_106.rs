// Generated macro for impl_106 (impl)
macro_rules! Depcrate_rgbimpl_106 {
() => {
// Module: crate::rgb
// Provides: {"impl_106"}
// Dependencies: {}
impl ANSIColorCode for Rgb { fn ansi_color_code (& self , target : TargetGround) -> String { format ! ("{};2;{};{};{}" , target . code () + 8 , self . r , self . g , self . b) } }
};
}
