// Generated macro for try_help (function)
macro_rules! Depcrate_error_formattry_help {
() => {
// Module: crate::error::format
// Provides: {"try_help"}
// Dependencies: {}
fn try_help (styled : & mut StyledStr , styles : & Styles , help : Option < & str >) { if let Some (help) = help { use std :: fmt :: Write as _ ; let literal = & styles . get_literal () ; let _ = write ! (styled , "\n\nFor more information, try '{literal}{help}{literal:#}'.\n" ,) ; } else { styled . push_str ("\n") ; } }
};
}
