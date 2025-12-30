// Generated macro for lookup_error (function)
macro_rules! Depcrate_errorlookup_error {
() => {
// Module: crate::error
// Provides: {"lookup_error"}
// Dependencies: {}
pub (crate) fn lookup_error (source : & str , step : & str , path : PathSlice , current : & Value) -> Error { let avail_str = if let Value :: Object (object_map) = current { let mut avail_str = " Available values at this level are " . to_string () ; for (i , key) in object_map . keys () . enumerate () { if i > 0 { avail_str . push_str (", ") ; } avail_str . push ('\'') ; avail_str . push_str (key) ; avail_str . push ('\'') ; } avail_str } else { "" . to_string () } ; let (line , column) = get_offset (source , step) ; Error :: RenderError { msg : format ! ("Failed to find value '{}' from path '{}'.{}" , step , path_to_str (path) , avail_str) , line , column , } }
};
}
