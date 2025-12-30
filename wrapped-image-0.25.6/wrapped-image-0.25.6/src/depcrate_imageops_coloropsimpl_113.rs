// Generated macro for impl_113 (impl)
macro_rules! Depcrate_imageops_coloropsimpl_113 {
() => {
// Module: crate::imageops::colorops
// Provides: {"impl_113"}
// Dependencies: {}
impl ColorMap for BiLevel { type Color = Luma < u8 > ; # [inline (always)] fn index_of (& self , color : & Luma < u8 >) -> usize { let luma = color . 0 ; if luma [0] > 127 { 1 } else { 0 } } # [inline (always)] fn lookup (& self , idx : usize) -> Option < Self :: Color > { match idx { 0 => Some ([0] . into ()) , 1 => Some ([255] . into ()) , _ => None , } } # [doc = " Indicate `NeuQuant` implements `lookup`."] fn has_lookup (& self) -> bool { true } # [inline (always)] fn map_color (& self , color : & mut Luma < u8 >) { let new_color = 0xFF * self . index_of (color) as u8 ; let luma = & mut color . 0 ; luma [0] = new_color ; } }
};
}
