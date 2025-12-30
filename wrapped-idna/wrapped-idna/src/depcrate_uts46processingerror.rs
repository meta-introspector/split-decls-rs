// Generated macro for ProcessingError (enum)
macro_rules! Depcrate_uts46ProcessingError {
() => {
// Module: crate::uts46
// Provides: {"ProcessingError"}
// Dependencies: {}
# [doc = " The failure outcome of [`Uts46::process`]"] # [derive (PartialEq , Eq , Copy , Clone , Debug)] pub enum ProcessingError { # [doc = " There was a validity error according to the chosen options."] # [doc = ""] # [doc = " In case of `Operation::ToAscii`, there is no output. Otherwise, output was written to the"] # [doc = " sink and the output contains at least one U+FFFD REPLACEMENT CHARACTER to denote an error."] ValidityError , # [doc = " The sink emitted [`core::fmt::Error`]. The partial output written to the sink must not"] # [doc = " be used."] SinkError , }
};
}
