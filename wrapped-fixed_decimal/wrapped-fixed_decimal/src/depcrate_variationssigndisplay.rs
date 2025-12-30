// Generated macro for SignDisplay (enum)
macro_rules! Depcrate_variationsSignDisplay {
() => {
// Module: crate::variations
// Provides: {"SignDisplay"}
// Dependencies: {}
# [doc = " Configuration for when to render the minus sign or plus sign."] # [doc = ""] # [doc = " **The primary definition of this type is in the [`fixed_decimal`](https://docs.rs/fixed_decimal) crate. Other ICU4X crates re-export it for convenience.**"] # [non_exhaustive] # [derive (Debug , Eq , PartialEq , Clone , Copy , Default)] pub enum SignDisplay { # [doc = " Render the sign according to locale preferences. In most cases, this means a minus sign"] # [doc = " will be shown on negative numbers, and no sign will be shown on positive numbers."] # [default] Auto , # [doc = " Do not display the sign. Positive and negative numbers are indistinguishable."] Never , # [doc = " Show a minus sign on negative numbers and a plus sign on positive numbers, including zero."] Always , # [doc = " Show a minus sign on negative numbers and a plus sign on positive numbers, except do not"] # [doc = " show any sign on positive or negative zero."] ExceptZero , # [doc = " Show a minus sign on strictly negative numbers. Do not show a sign on positive numbers or"] # [doc = " on positive or negative zero."] # [doc = ""] # [doc = " This differs from [`Auto`](SignDisplay::Auto) in that it does not render a sign on negative zero."] Negative , }
};
}
