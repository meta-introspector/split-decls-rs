// Generated macro for ReverseCoordTranslate (trait)
macro_rules! Depcrate_coord_translateReverseCoordTranslate {
() => {
// Module: crate::coord::translate
// Provides: {"ReverseCoordTranslate"}
// Dependencies: {}
# [doc = " The trait indicates that the coordinate system supports reverse transform"] # [doc = " This is useful when we need an interactive plot, thus we need to map the event"] # [doc = " from the backend coordinate to the logical coordinate"] pub trait ReverseCoordTranslate : CoordTranslate { # [doc = " Reverse translate the coordinate from the drawing coordinate to the"] # [doc = " logic coordinate."] # [doc = " Note: the return value is an option, because it's possible that the drawing"] # [doc = " coordinate isn't able to be represented in te guest coordinate system"] fn reverse_translate (& self , input : BackendCoord) -> Option < Self :: From > ; }
};
}
