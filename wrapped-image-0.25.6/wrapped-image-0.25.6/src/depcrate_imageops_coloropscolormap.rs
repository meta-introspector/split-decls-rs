// Generated macro for ColorMap (trait)
macro_rules! Depcrate_imageops_coloropsColorMap {
() => {
// Module: crate::imageops::colorops
// Provides: {"ColorMap"}
// Dependencies: {}
# [doc = " A color map"] pub trait ColorMap { # [doc = " The color type on which the map operates on"] type Color ; # [doc = " Returns the index of the closest match of `color`"] # [doc = " in the color map."] fn index_of (& self , color : & Self :: Color) -> usize ; # [doc = " Looks up color by index in the color map.  If `idx` is out of range for the color map, or"] # [doc = " `ColorMap` doesn't implement `lookup` `None` is returned."] fn lookup (& self , index : usize) -> Option < Self :: Color > { let _ = index ; None } # [doc = " Determine if this implementation of `ColorMap` overrides the default `lookup`."] fn has_lookup (& self) -> bool { false } # [doc = " Maps `color` to the closest color in the color map."] fn map_color (& self , color : & mut Self :: Color) ; }
};
}
