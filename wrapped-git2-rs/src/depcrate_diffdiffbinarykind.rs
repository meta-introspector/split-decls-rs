// Generated macro for DiffBinaryKind (enum)
macro_rules! Depcrate_diffDiffBinaryKind {
() => {
// Module: crate::diff
// Provides: {"DiffBinaryKind"}
// Dependencies: {}
# [doc = " When producing a binary diff, the binary data returned will be"] # [doc = " either the deflated full (\"literal\") contents of the file, or"] # [doc = " the deflated binary delta between the two sides (whichever is"] # [doc = " smaller)."] # [derive (Copy , Clone , Debug)] pub enum DiffBinaryKind { # [doc = " There is no binary delta"] None , # [doc = " The binary data is the literal contents of the file"] Literal , # [doc = " The binary data is the delta from one side to the other"] Delta , }
};
}
