// Generated macro for impl_553 (impl)
macro_rules! Depcrate_error_formatimpl_553 {
() => {
// Module: crate::error::format
// Provides: {"impl_553"}
// Dependencies: {}
impl ErrorFormatter for KindFormatter { fn format_error (error : & crate :: error :: Error < Self >) -> StyledStr { use std :: fmt :: Write as _ ; let styles = & error . inner . styles ; let mut styled = StyledStr :: new () ; start_error (& mut styled , styles) ; if let Some (msg) = error . kind () . as_str () { styled . push_str (msg) ; } else if let Some (source) = error . inner . source . as_ref () { let _ = write ! (styled , "{source}") ; } else { styled . push_str ("unknown cause") ; } styled . push_str ("\n") ; styled } }
};
}
