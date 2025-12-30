// Generated macro for impl_115 (impl)
macro_rules! Depcrate_impls_core__cellimpl_115 {
() => {
// Module: crate::impls::core_::cell
// Provides: {"impl_115"}
// Dependencies: {}
impl < T > Format for core :: cell :: RefCell < T > where T : Format , { default_format ! () ; # [inline] fn _format_tag () -> Str { internp ! ("RefCell {{ value: <borrowed> }}|RefCell {{ value: {=?} }}") } # [inline] fn _format_data (& self) { match self . try_borrow () { Err (_) => export :: u8 (& 0) , Ok (x) => { export :: u8 (& 1) ; export :: istr (& T :: _format_tag ()) ; x . _format_data () } } } }
};
}
