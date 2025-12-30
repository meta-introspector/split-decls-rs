// Generated macro for CoordMapper (trait)
macro_rules! Depcrate_elementCoordMapper {
() => {
// Module: crate::element
// Provides: {"CoordMapper"}
// Dependencies: {}
# [doc = " Useful to translate from guest coordinates to backend coordinates"] pub trait CoordMapper { # [doc = " Specifies the output data from the translation"] type Output ; # [doc = " Performs the translation from guest coordinates to backend coordinates"] fn map < CT : CoordTranslate > (coord_trans : & CT , from : & CT :: From , rect : & Rect) -> Self :: Output ; }
};
}
