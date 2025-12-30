// Generated macro for CloneUsage (struct)
macro_rules! Depcrate_redundant_cloneCloneUsage {
() => {
// Module: crate::redundant_clone
// Provides: {"CloneUsage"}
// Dependencies: {}
# [derive (Debug , Default)] struct CloneUsage { # [doc = " The first location where the cloned value is used, if any."] cloned_use_loc : MirLocalUsage , # [doc = " The first location where the cloned value is consumed or mutated, if any."] cloned_consume_or_mutate_loc : Option < mir :: Location > , # [doc = " Whether the clone value is mutated."] clone_consumed_or_mutated : bool , }
};
}
