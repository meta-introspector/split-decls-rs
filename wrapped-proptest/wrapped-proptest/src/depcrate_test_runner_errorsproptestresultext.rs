// Generated macro for ProptestResultExt (trait)
macro_rules! Depcrate_test_runner_errorsProptestResultExt {
() => {
// Module: crate::test_runner::errors
// Provides: {"ProptestResultExt"}
// Dependencies: {}
# [doc = " Extension trait for `Result<T, E>` to provide additional functionality"] # [doc = " specifically for prop test cases."] pub trait ProptestResultExt < T , E > : private :: Sealed { # [doc = " Converts a `Result<T, E>` into a `Result<T, TestCaseError>`, where the"] # [doc = " `Err` case is transformed into a `TestCaseError::Reject`."] # [doc = ""] # [doc = " This is intended to be used like the [`prop_assume!`] macro, but for"] # [doc = " fallible computations. If the result is `Err`, the test input is rejected"] # [doc = " and a new input will be generated."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```"] # [doc = " use proptest::prelude::*;"] # [doc = ""] # [doc = " fn test_conversion(a: i32) -> Result<(), TestCaseError> {"] # [doc = "     // Reject the case if `a` cannot be converted to u8 (e.g., negative values)"] # [doc = "     let _unsigned: u8 = a.try_into().prop_assume_ok()?;"] # [doc = "     // ...rest of test..."] # [doc = "     Ok(())"] # [doc = " }"] # [doc = ""] # [doc = " proptest! {"] # [doc = "   #[test]"] # [doc = "   fn test_that_only_works_with_positive_integers(a in -10i32..10i32) {"] # [doc = "     test_conversion(a)?;"] # [doc = "   }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [`prop_assume!`]: crate::prop_assume"] fn prop_assume_ok (self) -> Result < T , TestCaseError > where E : fmt :: Debug ; }
};
}
