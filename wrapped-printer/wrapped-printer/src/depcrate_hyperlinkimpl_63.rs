// Generated macro for impl_63 (impl)
macro_rules! Depcrate_hyperlinkimpl_63 {
() => {
// Module: crate::hyperlink
// Provides: {"impl_63"}
// Dependencies: {}
impl Part { # [doc = " Interpolate this part using the given `env` and `values`, and write"] # [doc = " the result of interpolation to the buffer provided."] fn interpolate_to (& self , env : & HyperlinkEnvironment , values : & Values , dest : & mut Vec < u8 > ,) { match * self { Part :: Text (ref text) => dest . extend_from_slice (text) , Part :: Host => dest . extend_from_slice (env . host . as_ref () . map (| s | s . as_bytes ()) . unwrap_or (b"") ,) , Part :: WSLPrefix => dest . extend_from_slice (env . wsl_prefix . as_ref () . map (| s | s . as_bytes ()) . unwrap_or (b"") ,) , Part :: Path => dest . extend_from_slice (& values . path . 0) , Part :: Line => { let line = DecimalFormatter :: new (values . line . unwrap_or (1)) ; dest . extend_from_slice (line . as_bytes ()) ; } Part :: Column => { let column = DecimalFormatter :: new (values . column . unwrap_or (1)) ; dest . extend_from_slice (column . as_bytes ()) ; } } } }
};
}
