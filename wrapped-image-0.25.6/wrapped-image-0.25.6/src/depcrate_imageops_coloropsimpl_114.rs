// Generated macro for impl_114 (impl)
macro_rules! Depcrate_imageops_coloropsimpl_114 {
() => {
// Module: crate::imageops::colorops
// Provides: {"impl_114"}
// Dependencies: {}
# [cfg (feature = "color_quant")] impl ColorMap for color_quant :: NeuQuant { type Color = crate :: color :: Rgba < u8 > ; # [inline (always)] fn index_of (& self , color : & Self :: Color) -> usize { self . index_of (color . channels ()) } # [inline (always)] fn lookup (& self , idx : usize) -> Option < Self :: Color > { self . lookup (idx) . map (| p | p . into ()) } # [doc = " Indicate NeuQuant implements `lookup`."] fn has_lookup (& self) -> bool { true } # [inline (always)] fn map_color (& self , color : & mut Self :: Color) { self . map_pixel (color . channels_mut ()) } }
};
}
