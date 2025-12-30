// Generated macro for impl_1021 (impl)
macro_rules! Depcrate_validation_test_harnessimpl_1021 {
() => {
// Module: crate::validation::test_harness
// Provides: {"impl_1021"}
// Dependencies: {}
impl < S > FromInputValue < S > for DogCommand where S : ScalarValue , { type Error = & 'static str ; fn from_input_value (v : & InputValue < S >) -> Result < DogCommand , Self :: Error > { match v . as_enum_value () { Some ("SIT") => Ok (DogCommand :: Sit) , Some ("HEEL") => Ok (DogCommand :: Heel) , Some ("DOWN") => Ok (DogCommand :: Down) , _ => Err ("Unknown DogCommand") , } } }
};
}
