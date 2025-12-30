// Generated macro for get_def (function)
macro_rules! Depcrate_unused_unitget_def {
() => {
// Module: crate::unused_unit
// Provides: {"get_def"}
// Dependencies: {}
# [must_use] fn get_def (span : Span) -> Option < Span > { if span . from_expansion () { Some (span . ctxt () . outer_expn_data () . def_site) } else { None } }
};
}
