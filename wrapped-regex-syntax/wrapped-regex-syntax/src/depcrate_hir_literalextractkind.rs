// Generated macro for ExtractKind (enum)
macro_rules! Depcrate_hir_literalExtractKind {
() => {
// Module: crate::hir::literal
// Provides: {"ExtractKind"}
// Dependencies: {}
# [doc = " The kind of literals to extract from an [`Hir`] expression."] # [doc = ""] # [doc = " The default extraction kind is `Prefix`."] # [non_exhaustive] # [derive (Clone , Debug)] pub enum ExtractKind { # [doc = " Extracts only prefix literals from a regex."] Prefix , # [doc = " Extracts only suffix literals from a regex."] # [doc = ""] # [doc = " Note that the sequence returned by suffix literals currently may"] # [doc = " not correctly represent leftmost-first or \"preference\" order match"] # [doc = " semantics."] Suffix , }
};
}
