// Generated macro for impl_520 (impl)
macro_rules! Depcrate_types_containersimpl_520 {
() => {
// Module: crate::types::containers
// Provides: {"impl_520"}
// Dependencies: {}
impl < S : ScalarValue , T : FromInputValue < S > > FromInputValue < S > for Vec < T > { type Error = FromInputValueVecError < T , S > ; fn from_input_value (v : & InputValue < S >) -> Result < Self , Self :: Error > { match v { InputValue :: List (l) => l . iter () . map (| i | i . item . convert () . map_err (FromInputValueVecError :: Item)) . collect () , InputValue :: Null => Err (FromInputValueVecError :: Null) , other => other . convert () . map (| e | vec ! [e]) . map_err (FromInputValueVecError :: Item) , } } }
};
}
