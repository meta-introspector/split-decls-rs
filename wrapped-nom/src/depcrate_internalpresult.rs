// Generated macro for PResult (type)
macro_rules! Depcrate_internalPResult {
() => {
// Module: crate::internal
// Provides: {"PResult"}
// Dependencies: {}
# [doc = " Parser result type"] # [doc = ""] # [doc = " * `Ok` branch: a tuple of the remaining input data, and the output value."] # [doc = "   The output value is of the `O` type if the output mode was [Emit], and `()`"] # [doc = "   if the mode was [Check]"] # [doc = " * `Err` branch: an error of the `E` type if the erroor mode was [Emit], and `()`"] # [doc = "   if the mode was [Check]"] pub type PResult < OM , I , O , E > = Result < (I , < < OM as OutputMode > :: Output as Mode > :: Output < O >) , Err < E , < < OM as OutputMode > :: Error as Mode > :: Output < E > > , > ;
};
}
