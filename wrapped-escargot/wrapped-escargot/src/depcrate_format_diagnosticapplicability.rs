// Generated macro for Applicability (enum)
macro_rules! Depcrate_format_diagnosticApplicability {
() => {
// Module: crate::format::diagnostic
// Provides: {"Applicability"}
// Dependencies: {}
# [doc = " Whether a suggestion can be safely applied."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Serialize , Deserialize)] pub enum Applicability { # [doc = " The suggested replacement can be applied automatically safely"] MachineApplicable , # [doc = " The suggested replacement has placeholders that will need to be manually"] # [doc = " replaced."] HasPlaceholders , # [doc = " The suggested replacement may be incorrect in some circumstances. Needs"] # [doc = " human review."] MaybeIncorrect , # [doc = " The suggested replacement will probably not work."] Unspecified , # [cfg (not (feature = "strict_unstable"))] # [doc (hidden)] # [serde (other)] Unknown , }
};
}
