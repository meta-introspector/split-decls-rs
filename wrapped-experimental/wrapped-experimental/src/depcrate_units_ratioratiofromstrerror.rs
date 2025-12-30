// Generated macro for RatioFromStrError (enum)
macro_rules! Depcrate_units_ratioRatioFromStrError {
() => {
// Module: crate::units::ratio
// Provides: {"RatioFromStrError"}
// Dependencies: {}
# [doc = " The ratio string is invalid and cannot be parsed."] # [derive (Debug , PartialEq , displaydoc :: Display)] pub enum RatioFromStrError { # [doc = " The ratio string is divided by zero."] DivisionByZero , # [doc = " The ratio string contains multiple slashes."] # [doc = ""] # [doc = " For example, \"1/2/3\"."] # [displaydoc ("The ratio string contains multiple slashes")] MultipleSlashes , # [doc = " The ratio string contains non-numeric characters in fractions."] # [doc = ""] # [doc = " For example, \"1/2A\"."] # [displaydoc ("The ratio string contains non-numeric characters in fractions")] NonNumericCharactersInFractions , # [doc = " The ratio string contains multiple scientific notations."] # [doc = ""] # [doc = " For example, \"1.5E6E6\"."] # [displaydoc ("The ratio string contains multiple scientific notations")] MultipleScientificNotations , # [doc = " The ratio string contains multiple decimal points."] # [doc = ""] # [doc = " For example, \"1.5.6\"."] # [displaydoc ("The ratio string contains multiple decimal points")] MultipleDecimalPoints , # [doc = " The exponent part of the ratio string is not an integer."] # [doc = ""] # [doc = " For example, \"1.5E6.5\"."] # [displaydoc ("The exponent part of the ratio string is not an integer")] ExponentPartIsNotAnInteger , # [doc = " The ratio string is deficient in some other way."] ParsingBigIntError (num_bigint :: ParseBigIntError) , }
};
}
