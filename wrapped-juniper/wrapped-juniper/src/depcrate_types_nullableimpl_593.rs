// Generated macro for impl_593 (impl)
macro_rules! Depcrate_types_nullableimpl_593 {
() => {
// Module: crate::types::nullable
// Provides: {"impl_593"}
// Dependencies: {}
impl < S , T : FromInputValue < S > > FromInputValue < S > for Nullable < T > { type Error = < T as FromInputValue < S > > :: Error ; fn from_input_value (v : & InputValue < S >) -> Result < Self , Self :: Error > { match v { & InputValue :: Null => Ok (Self :: ExplicitNull) , v => v . convert () . map (Self :: Some) , } } fn from_implicit_null () -> Result < Self , Self :: Error > { Ok (Self :: ImplicitNull) } }
};
}
