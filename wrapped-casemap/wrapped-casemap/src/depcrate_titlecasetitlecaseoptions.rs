// Generated macro for TitlecaseOptions (struct)
macro_rules! Depcrate_titlecaseTitlecaseOptions {
() => {
// Module: crate::titlecase
// Provides: {"TitlecaseOptions"}
// Dependencies: {}
# [doc = " Various options for controlling titlecasing"] # [doc = ""] # [doc = " See docs of [`TitlecaseMapper`] for examples."] # [non_exhaustive] # [derive (Copy , Clone , Default , PartialEq , Eq , Hash , Debug)] pub struct TitlecaseOptions { # [doc = " How to handle the rest of the string once the head of the"] # [doc = " string has been titlecased"] # [doc = ""] # [doc = " Default is [`TrailingCase::Lower`]"] pub trailing_case : Option < TrailingCase > , # [doc = " Whether to start casing at the beginning of the string or at the first"] # [doc = " relevant character."] # [doc = ""] # [doc = " Default is [`LeadingAdjustment::Auto`]"] pub leading_adjustment : Option < LeadingAdjustment > , }
};
}
