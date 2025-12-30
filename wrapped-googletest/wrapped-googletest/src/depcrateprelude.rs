// Generated macro for prelude (module)
macro_rules! Depcrateprelude {
() => {
// Module: crate
// Provides: {"prelude"}
// Dependencies: {}
# [doc = " Re-exports of the symbols in this crate which are most likely to be used."] # [doc = ""] # [doc = " This includes:"] # [doc = "  * All assertion macros,"] # [doc = "  * Traits and type definitions normally used by tests, and"] # [doc = "  * All built-in matchers."] # [doc = ""] # [doc = " Typically, one imports everything in the prelude in one's test module:"] # [doc = ""] # [doc = " ```"] # [doc = " mod tests {"] # [doc = "     use googletest::prelude::*;"] # [doc = " }"] # [doc = " ```"] pub mod prelude { pub use super :: fixtures :: { ConsumableFixture , Fixture , FixtureOf , StaticFixture } ; pub use super :: gtest ; pub use super :: matcher :: { Matcher , MatcherBase } ; pub use super :: matchers :: * ; pub use super :: verify_current_test_outcome ; pub use super :: GoogleTestSupport ; pub use super :: OrFail ; pub use super :: Result ; pub use super :: { add_failure , add_failure_at , assert_pred , assert_that , expect_eq , expect_false , expect_float_eq , expect_ge , expect_gt , expect_le , expect_lt , expect_ne , expect_near , expect_pred , expect_that , expect_true , fail , succeed , verify_eq , verify_false , verify_float_eq , verify_ge , verify_gt , verify_le , verify_lt , verify_ne , verify_near , verify_pred , verify_that , verify_true , } ; }
};
}
