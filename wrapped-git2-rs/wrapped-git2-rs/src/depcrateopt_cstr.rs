// Generated macro for opt_cstr (function)
macro_rules! Depcrateopt_cstr {
() => {
// Module: crate
// Provides: {"opt_cstr"}
// Dependencies: {}
fn opt_cstr < T : IntoCString > (o : Option < T >) -> Result < Option < CString > , Error > { match o { Some (s) => s . into_c_string () . map (Some) , None => Ok (None) , } }
};
}
