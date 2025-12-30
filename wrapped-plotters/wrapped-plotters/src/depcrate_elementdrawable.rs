// Generated macro for Drawable (trait)
macro_rules! Depcrate_elementDrawable {
() => {
// Module: crate::element
// Provides: {"Drawable"}
// Dependencies: {}
# [doc = " The trait indicates we are able to draw it on a drawing area"] pub trait Drawable < DB : DrawingBackend , CM : CoordMapper = BackendCoordOnly > { # [doc = " Actually draws the element. The key points is already translated into the"] # [doc = " image coordinate and can be used by DC directly"] fn draw < I : Iterator < Item = CM :: Output > > (& self , pos : I , backend : & mut DB , parent_dim : (u32 , u32) ,) -> Result < () , DrawingErrorKind < DB :: ErrorType > > ; }
};
}
