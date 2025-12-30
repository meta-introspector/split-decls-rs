// Generated macro for impl_52 (impl)
macro_rules! Depcrate_core_colorimpl_52 {
() => {
// Module: crate::core::color
// Provides: {"impl_52"}
// Dependencies: {}
impl Color { pub fn new (hex : u32) -> Self { Self { color : hex } } pub fn transparent () -> Self { Color :: new (0x00000000) } pub fn fast (name : & str) -> Color { if let Option :: Some (c) = Self :: from_name (name) { c } else { Color :: new (0x000000ff) } } pub fn from_name (name : & str) -> Option < Color > { for pair in KNOWN_COLORS { if name == pair . 0 { return Some (Color :: new ((pair . 1 << 8) + 0xff)) ; } } if name . starts_with ('#') { let name = name . trim_start_matches ('#') ; if let Result :: Ok (color) = u32 :: from_str_radix (name , 16) { if name . len () <= 7 { return Some (Color :: new ((color << 8) + 0xff)) ; } else { return Some (Color :: new (color)) ; } } } None } pub fn to_web_color (& self) -> String { format ! ("#{:08x}" , self . color) } }
};
}
