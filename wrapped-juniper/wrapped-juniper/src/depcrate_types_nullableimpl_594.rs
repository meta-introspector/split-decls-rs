// Generated macro for impl_594 (impl)
macro_rules! Depcrate_types_nullableimpl_594 {
() => {
// Module: crate::types::nullable
// Provides: {"impl_594"}
// Dependencies: {}
impl < S , T > ToInputValue < S > for Nullable < T > where T : ToInputValue < S > , { fn to_input_value (& self) -> InputValue < S > { match self { Self :: Some (v) => v . to_input_value () , _ => InputValue :: null () , } } }
};
}
