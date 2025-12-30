// Generated macro for Edit (enum)
macro_rules! Depcrate_matcher_support_edit_distanceEdit {
() => {
// Module: crate::matcher_support::edit_distance
// Provides: {"Edit"}
// Dependencies: {}
# [doc = " An edit operation on two sequences of `T`."] # [derive (Debug , Clone)] pub (crate) enum Edit < T > { # [doc = " An extra `T` was added to the actual sequence."] ExtraActual (T) , # [doc = " An extra `T` was added to the expected sequence."] ExtraExpected (T) , # [doc = " An element was added to each sequence."] Both (T) , # [doc = " Additional (unlisted) elements are present in the actual sequence."] # [doc = ""] # [doc = " This is only output in the mode [`Mode::Prefix`]. Its presence precludes"] # [doc = " reconstructing the actual sequence from the expected sequence."] AdditionalActual , }
};
}
