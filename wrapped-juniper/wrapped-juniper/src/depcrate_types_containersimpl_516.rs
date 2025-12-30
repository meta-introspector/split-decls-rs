// Generated macro for impl_516 (impl)
macro_rules! Depcrate_types_containersimpl_516 {
() => {
// Module: crate::types::containers
// Provides: {"impl_516"}
// Dependencies: {}
impl < S , T > ToInputValue < S > for Option < T > where T : ToInputValue < S > , { fn to_input_value (& self) -> InputValue < S > { match self { Some (v) => v . to_input_value () , None => InputValue :: Null , } } }
};
}
