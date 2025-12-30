// Generated macro for impl_120 (impl)
macro_rules! Depcrate_pathimpl_120 {
() => {
// Module: crate::path
// Provides: {"impl_120"}
// Dependencies: {}
impl < W : WriteColor > PathPrinter < W > { # [doc = " Write the given path to the underlying writer."] pub fn write (& mut self , path : & Path) -> io :: Result < () > { let ppath = PrinterPath :: new (path . as_ref ()) . with_separator (self . config . separator) ; if ! self . wtr . supports_color () { self . wtr . write_all (ppath . as_bytes ()) ? ; } else { let status = self . start_hyperlink (& ppath) ? ; self . wtr . set_color (self . config . colors . path ()) ? ; self . wtr . write_all (ppath . as_bytes ()) ? ; self . wtr . reset () ? ; self . interpolator . finish (status , & mut self . wtr) ? ; } self . wtr . write_all (& [self . config . terminator]) } # [doc = " Starts a hyperlink span when applicable."] fn start_hyperlink (& mut self , path : & PrinterPath ,) -> io :: Result < hyperlink :: InterpolatorStatus > { let Some (hyperpath) = path . as_hyperlink () else { return Ok (hyperlink :: InterpolatorStatus :: inactive ()) ; } ; let values = hyperlink :: Values :: new (hyperpath) ; self . interpolator . begin (& values , & mut self . wtr) } }
};
}
