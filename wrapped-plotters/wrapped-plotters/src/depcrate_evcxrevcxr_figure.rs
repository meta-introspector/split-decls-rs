// Generated macro for evcxr_figure (function)
macro_rules! Depcrate_evcxrevcxr_figure {
() => {
// Module: crate::evcxr
// Provides: {"evcxr_figure"}
// Dependencies: {}
# [doc = " Start drawing an evcxr figure"] pub fn evcxr_figure < Draw : FnOnce (DrawingArea < SVGBackend , Shift >) -> Result < () , Box < dyn std :: error :: Error > > , > (size : (u32 , u32) , draw : Draw ,) -> SVGWrapper { let mut buffer = "" . to_string () ; let root = SVGBackend :: with_string (& mut buffer , size) . into_drawing_area () ; draw (root) . expect ("Drawing failure") ; SVGWrapper (buffer , "" . to_string ()) }
};
}
