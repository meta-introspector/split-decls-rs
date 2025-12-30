// Generated macro for macro_60 (macro)
macro_rules! Depcrate_into_urlmacro_60 {
() => {
// Module: crate::into_url
// Provides: {"macro_60"}
// Dependencies: {}
if_hyper ! { pub (crate) fn try_uri (url : & Url) -> crate :: Result < http :: Uri > { url . as_str () . parse () . map_err (| _ | crate :: error :: url_invalid_uri (url . clone ())) } }
};
}
