// Generated macro for impl_8083 (impl)
macro_rules! Depcrate_non_send_fields_in_send_tyimpl_8083 {
() => {
// Module: crate::non_send_fields_in_send_ty
// Provides: {"impl_8083"}
// Dependencies: {}
impl NonSendField < '_ > { fn generic_params_string (& self) -> String { self . generic_params . iter () . map (ToString :: to_string) . collect :: < Vec < _ > > () . join (", ") } }
};
}
