// Generated macro for OrFail (trait)
macro_rules! DepcrateOrFail {
() => {
// Module: crate
// Provides: {"OrFail"}
// Dependencies: {}
# [doc = " Provides an extension method for converting an arbitrary type into"] # [doc = " `googletest`'s [`Result`] type."] # [doc = ""] # [doc = " A type can implement this trait to provide an easy way to return immediately"] # [doc = " from a test in conjunction with the `?` operator. This is useful for"] # [doc = " [`Option`] and [`Result`][std::result::Result] types whose `Result::Err`"] # [doc = " variant does not implement [`std::error::Error`]."] # [doc = ""] # [doc = " If `Result::Err` implements [`std::error::Error`] you can just use the `?`"] # [doc = " operator directly."] # [doc = ""] # [doc = " ```ignore"] # [doc = " #[test]"] # [doc = " fn should_work() -> googletest::Result<()> {"] # [doc = "     let value = something_which_can_fail().or_fail()?;"] # [doc = "     let value = something_which_can_fail_with_option().or_fail()?;"] # [doc = "     ..."] # [doc = " }"] # [doc = ""] # [doc = " fn something_which_can_fail() -> std::result::Result<T, String> { ... }"] # [doc = " fn something_which_can_fail_with_option() -> Option<T> { ... }"] # [doc = " ```"] pub trait OrFail { # [doc = " The success type of the test result."] type Output ; # [doc = " Converts a value into a [`Result`] containing"] # [doc = " either the [`Self::Output`] type or a [`TestAssertionFailure`]."] # [doc = ""] # [doc = " The most frequently used implementations convert"] # [doc = " `Result<T, E>` into `Result<T, TestAssertionFailure>` and"] # [doc = " `Option<T>` into `Result<T, TestAssertionFailure>`."] fn or_fail (self) -> Result < Self :: Output > ; }
};
}
