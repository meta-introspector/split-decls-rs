// Generated macro for evcxr_bitmap_figure (function)
macro_rules! Depcrate_evcxrevcxr_bitmap_figure {
() => {
// Module: crate::evcxr
// Provides: {"evcxr_bitmap_figure"}
// Dependencies: {}
# [doc = " Start drawing an evcxr figure"] # [cfg (feature = "evcxr_bitmap")] # [cfg_attr (doc_cfg , doc (cfg (feature = "evcxr_bitmap")))] pub fn evcxr_bitmap_figure < Draw : FnOnce (DrawingArea < BitMapBackend , Shift >) -> Result < () , Box < dyn std :: error :: Error > > , > (size : (u32 , u32) , draw : Draw ,) -> SVGWrapper { const PIXEL_SIZE : usize = 3 ; let mut buf = vec ! [0 ; (size . 0 as usize) * (size . 1 as usize) * PIXEL_SIZE] ; let root = BitMapBackend :: with_buffer (& mut buf , size) . into_drawing_area () ; draw (root) . expect ("Drawing failure") ; let mut buffer = "" . to_string () ; { let mut svg_root = SVGBackend :: with_string (& mut buffer , size) ; svg_root . blit_bitmap ((0 , 0) , size , & buf) . expect ("Failure converting to SVG") ; } SVGWrapper (buffer , "" . to_string ()) }
};
}
