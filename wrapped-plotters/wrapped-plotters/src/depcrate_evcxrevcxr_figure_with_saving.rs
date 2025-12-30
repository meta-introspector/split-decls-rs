// Generated macro for evcxr_figure_with_saving (function)
macro_rules! Depcrate_evcxrevcxr_figure_with_saving {
() => {
// Module: crate::evcxr
// Provides: {"evcxr_figure_with_saving"}
// Dependencies: {}
# [doc = " An evcxr figure that can save to the local file system and render in a notebook."] pub fn evcxr_figure_with_saving < Draw : FnOnce (DrawingArea < SVGBackend , Shift >) -> Result < () , Box < dyn std :: error :: Error > > , > (filename : & str , size : (u32 , u32) , draw : Draw ,) -> SVGWrapper { let mut buffer = "" . to_string () ; let root = SVGBackend :: with_string (& mut buffer , size) . into_drawing_area () ; draw (root) . expect ("Drawing failure") ; let mut file = File :: create (filename) . expect ("Unable to create file") ; file . write_all (buffer . as_bytes ()) . expect ("Unable to write data") ; SVGWrapper (buffer , "" . to_string ()) }
};
}
