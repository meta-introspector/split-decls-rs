// Generated macro for macro_19 (macro)
macro_rules! Depcrate_errorsmacro_19 {
() => {
// Module: crate::errors
// Provides: {"macro_19"}
// Dependencies: {}
simple ! { # [doc = " Error returned by [`Utf16CharDecoder`](../iterator/struct.Utf16CharMerger.html#impl-Iterator)"] # [doc = " when it encounters an invalid sequence."] Utf16PairError { # [doc = " A trailing surrogate was not preceeded by a leading surrogate."] UnexpectedTrailingSurrogate => "a trailing surrogate was not preceeded by a leading surrogate" , # [doc = " A leading surrogate was followed by an unit that was not a trailing surrogate."] UnmatchedLeadingSurrogate => "a leading surrogate was followed by an unit that was not a trailing surrogate" , # [doc = " A trailing surrogate was expected when the end was reached."] Incomplete => "a trailing surrogate was expected when the end was reached" , } }
};
}
