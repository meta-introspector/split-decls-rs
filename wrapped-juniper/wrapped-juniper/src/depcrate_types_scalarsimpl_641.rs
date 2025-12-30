// Generated macro for impl_641 (impl)
macro_rules! Depcrate_types_scalarsimpl_641 {
() => {
// Module: crate::types::scalars
// Provides: {"impl_641"}
// Dependencies: {}
impl < S > ToInputValue < S > for str where Self : ToScalarValue < S > , { fn to_input_value (& self) -> InputValue < S > { InputValue :: Scalar (self . to_scalar_value ()) } }
};
}
