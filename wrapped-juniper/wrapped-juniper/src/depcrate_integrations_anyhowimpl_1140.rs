// Generated macro for impl_1140 (impl)
macro_rules! Depcrate_integrations_anyhowimpl_1140 {
() => {
// Module: crate::integrations::anyhow
// Provides: {"impl_1140"}
// Dependencies: {}
impl < S : ScalarValue > IntoFieldError < S > for anyhow :: Error { fn into_field_error (self) -> FieldError < S > { # [cfg (any (nightly , feature = "backtrace"))] let extensions = { let backtrace = self . backtrace () . to_string () ; if backtrace == "disabled backtrace" { Value :: Null } else { let mut obj = crate :: value :: Object :: with_capacity (1) ; _ = obj . add_field ("backtrace" , Value :: List (backtrace . split ('\n') . map (| line | Value :: Scalar (line . to_owned () . into ())) . collect () ,) ,) ; Value :: Object (obj) } } ; # [cfg (not (any (nightly , feature = "backtrace")))] let extensions = Value :: Null ; FieldError :: new (self , extensions) } }
};
}
