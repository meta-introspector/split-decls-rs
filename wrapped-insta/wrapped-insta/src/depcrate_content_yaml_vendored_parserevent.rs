// Generated macro for Event (enum)
macro_rules! Depcrate_content_yaml_vendored_parserEvent {
() => {
// Module: crate::content::yaml::vendored::parser
// Provides: {"Event"}
// Dependencies: {}
# [doc = " [`Event`] is used with the low-level event base parsing API,"] # [doc = " see [`EventReceiver`] trait."] # [derive (Clone , PartialEq , Debug , Eq)] pub enum Event { # [doc = " Reserved for internal use"] StreamStart , StreamEnd , DocumentStart , DocumentEnd , # [doc = " Refer to an anchor ID"] Alias (usize) , # [doc = " Value, style, anchor ID, tag"] Scalar (String , TScalarStyle , usize , Option < TokenType >) , # [doc = " Anchor ID"] SequenceStart (usize) , SequenceEnd , # [doc = " Anchor ID"] MappingStart (usize) , MappingEnd , }
};
}
