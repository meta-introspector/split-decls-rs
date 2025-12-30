// Generated macro for backend (module)
macro_rules! Depcratebackend {
() => {
// Module: crate
// Provides: {"backend"}
// Dependencies: {}
# [doc = " This module contains some useful re-export of backend related types."] pub mod backend { pub use plotters_backend :: DrawingBackend ; # [cfg (feature = "bitmap_backend")] # [cfg_attr (doc_cfg , doc (cfg (feature = "bitmap_backend")))] pub use plotters_bitmap :: { bitmap_pixel :: { BGRXPixel , PixelFormat , RGBPixel } , BitMapBackend , } ; # [cfg (feature = "svg_backend")] # [cfg_attr (doc_cfg , doc (cfg (feature = "svg_backend")))] pub use plotters_svg :: SVGBackend ; }
};
}
