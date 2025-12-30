// Generated macro for scalar_meta (function)
macro_rules! Depcrate_parser_tests_valuescalar_meta {
() => {
// Module: crate::parser::tests::value
// Provides: {"scalar_meta"}
// Dependencies: {}
fn scalar_meta < T > (name : & 'static str) -> MetaType where T : FromInputValue < DefaultScalarValue > + ParseScalarValue < DefaultScalarValue > , T :: Error : IntoFieldError , { MetaType :: Scalar (ScalarMeta :: new :: < T > (name)) }
};
}
