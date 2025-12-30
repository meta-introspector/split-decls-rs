// Generated macro for Difference (enum)
macro_rules! Depcrate_differenceDifference {
() => {
// Module: crate::difference
// Provides: {"Difference"}
// Dependencies: {}
# [doc = " When printing out one coloured string followed by another, use one of"] # [doc = " these rules to figure out which *extra* control codes need to be sent."] # [derive (PartialEq , Clone , Copy , Debug)] pub enum Difference { # [doc = " Print out the control codes specified by this style to end up looking"] # [doc = " like the second string's styles."] ExtraStyles (Style) , # [doc = " Converting between these two is impossible, so just send a reset"] # [doc = " command and then the second string's styles."] Reset , # [doc = " The before style is exactly the same as the after style, so no further"] # [doc = " control codes need to be printed."] # [allow (clippy :: enum_variant_names)] NoDifference , }
};
}
