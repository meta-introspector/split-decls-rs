// Generated macro for ParallelVisitorBuilder (trait)
macro_rules! Depcrate_walkParallelVisitorBuilder {
() => {
// Module: crate::walk
// Provides: {"ParallelVisitorBuilder"}
// Dependencies: {}
# [doc = " A builder for constructing a visitor when using [`WalkParallel::visit`]."] # [doc = " The builder will be called for each thread started by `WalkParallel`. The"] # [doc = " visitor returned from each builder is then called for every directory"] # [doc = " entry."] pub trait ParallelVisitorBuilder < 's > { # [doc = " Create per-thread `ParallelVisitor`s for `WalkParallel`."] fn build (& mut self) -> Box < dyn ParallelVisitor + 's > ; }
};
}
