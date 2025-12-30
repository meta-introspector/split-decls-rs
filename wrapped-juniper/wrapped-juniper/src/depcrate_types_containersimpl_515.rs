// Generated macro for impl_515 (impl)
macro_rules! Depcrate_types_containersimpl_515 {
() => {
// Module: crate::types::containers
// Provides: {"impl_515"}
// Dependencies: {}
impl < S , T : FromInputValue < S > > FromInputValue < S > for Option < T > { type Error = T :: Error ; fn from_input_value (v : & InputValue < S >) -> Result < Self , Self :: Error > { match v { InputValue :: Null => Ok (None) , v => v . convert () . map (Some) , } } }
};
}
