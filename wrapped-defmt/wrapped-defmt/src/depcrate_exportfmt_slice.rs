// Generated macro for fmt_slice (function)
macro_rules! Depcrate_exportfmt_slice {
() => {
// Module: crate::export
// Provides: {"fmt_slice"}
// Dependencies: {}
# [doc = " Implementation detail"] pub fn fmt_slice < T : Format > (values : & [T]) { usize (& values . len ()) ; istr (& T :: _format_tag ()) ; for value in values { value . _format_data () ; } }
};
}
