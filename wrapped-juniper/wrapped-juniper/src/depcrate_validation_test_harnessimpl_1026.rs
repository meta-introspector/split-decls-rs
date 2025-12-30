// Generated macro for impl_1026 (impl)
macro_rules! Depcrate_validation_test_harnessimpl_1026 {
() => {
// Module: crate::validation::test_harness
// Provides: {"impl_1026"}
// Dependencies: {}
impl < S > FromInputValue < S > for FurColor where S : ScalarValue , { type Error = & 'static str ; fn from_input_value (v : & InputValue < S >) -> Result < FurColor , Self :: Error > { match v . as_enum_value () { Some ("BROWN") => Ok (FurColor :: Brown) , Some ("BLACK") => Ok (FurColor :: Black) , Some ("TAN") => Ok (FurColor :: Tan) , Some ("SPOTTED") => Ok (FurColor :: Spotted) , _ => Err ("Unknown FurColor") , } } }
};
}
