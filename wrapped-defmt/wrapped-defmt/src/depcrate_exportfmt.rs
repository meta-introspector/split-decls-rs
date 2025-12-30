// Generated macro for fmt (function)
macro_rules! Depcrate_exportfmt {
() => {
// Module: crate::export
// Provides: {"fmt"}
// Dependencies: {}
# [doc = " Implementation detail"] pub fn fmt < T : Format + ? Sized > (f : & T) { istr (& T :: _format_tag ()) ; f . _format_data () ; }
};
}
