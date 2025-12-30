// Generated macro for RGBAColor (struct)
macro_rules! Depcrate_style_colorRGBAColor {
() => {
// Module: crate::style::color
// Provides: {"RGBAColor"}
// Dependencies: {}
# [doc = " The RGBA representation of the color, Plotters use RGBA as the internal representation"] # [doc = " of color"] # [doc = ""] # [doc = " If you want to directly create a RGB color with transparency use [RGBColor::mix]"] # [derive (Copy , Clone , PartialEq , Debug , Default)] # [cfg_attr (feature = "serialization" , derive (Serialize , Deserialize))] pub struct RGBAColor (pub u8 , pub u8 , pub u8 , pub f64) ;
};
}
