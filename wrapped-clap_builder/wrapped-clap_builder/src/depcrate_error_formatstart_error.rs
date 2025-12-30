// Generated macro for start_error (function)
macro_rules! Depcrate_error_formatstart_error {
() => {
// Module: crate::error::format
// Provides: {"start_error"}
// Dependencies: {}
fn start_error (styled : & mut StyledStr , styles : & Styles) { use std :: fmt :: Write as _ ; let error = & styles . get_error () ; let _ = write ! (styled , "{error}error:{error:#} ") ; }
};
}
