// Generated macro for impl_188 (impl)
macro_rules! Depcrate_impls_core_impl_188 {
() => {
// Module: crate::impls::core_
// Provides: {"impl_188"}
// Dependencies: {}
impl < T , E > Format for Result < T , E > where T : Format , E : Format , { default_format ! () ; # [inline] fn _format_tag () -> Str { internp ! ("Err({=?})|Ok({=?})") } # [inline] fn _format_data (& self) { match self { Err (e) => { export :: u8 (& 0) ; export :: istr (& E :: _format_tag ()) ; e . _format_data () } Ok (x) => { export :: u8 (& 1) ; export :: istr (& T :: _format_tag ()) ; x . _format_data () } } } }
};
}
