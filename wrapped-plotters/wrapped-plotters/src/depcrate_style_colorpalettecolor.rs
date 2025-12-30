// Generated macro for PaletteColor (struct)
macro_rules! Depcrate_style_colorPaletteColor {
() => {
// Module: crate::style::color
// Provides: {"PaletteColor"}
// Dependencies: {}
# [doc = " A color in the given palette"] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug , Default)] # [cfg_attr (feature = "serialization" , derive (Serialize , Deserialize))] pub struct PaletteColor < P : Palette > (usize , PhantomData < P >) ;
};
}
