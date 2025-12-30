// Generated macro for url_scalar (module)
macro_rules! Depcrate_integrations_urlurl_scalar {
() => {
// Module: crate::integrations::url
// Provides: {"url_scalar"}
// Dependencies: {}
mod url_scalar { use super :: Url ; pub (super) fn from_input (s : & str) -> Result < Url , Box < str > > { Url :: parse (s) . map_err (| e | format ! ("Failed to parse `URL`: {e}") . into ()) } }
};
}
