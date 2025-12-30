// Generated macro for impl_187 (impl)
macro_rules! Depcrate_impls_core_impl_187 {
() => {
// Module: crate::impls::core_
// Provides: {"impl_187"}
// Dependencies: {}
impl < T > Format for Option < T > where T : Format , { default_format ! () ; # [inline] fn _format_tag () -> Str { internp ! ("None|Some({=?})") } # [inline] fn _format_data (& self) { match self { None => export :: u8 (& 0) , Some (x) => { export :: u8 (& 1) ; export :: istr (& T :: _format_tag ()) ; x . _format_data () } } } }
};
}
