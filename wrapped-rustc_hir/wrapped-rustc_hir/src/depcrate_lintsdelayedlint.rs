// Generated macro for DelayedLint (enum)
macro_rules! Depcrate_lintsDelayedLint {
() => {
// Module: crate::lints
// Provides: {"DelayedLint"}
// Dependencies: {}
# [doc = " During ast lowering, no lints can be emitted."] # [doc = " That is because lints attach to nodes either in the AST, or on the built HIR."] # [doc = " When attached to AST nodes, they're emitted just before building HIR,"] # [doc = " and then there's a gap where no lints can be emitted until HIR is done."] # [doc = " The variants in this enum represent lints that are temporarily stashed during"] # [doc = " AST lowering to be emitted once HIR is built."] # [derive (Clone , Debug , HashStable_Generic)] pub enum DelayedLint { AttributeParsing (AttributeLint < HirId >) , }
};
}
