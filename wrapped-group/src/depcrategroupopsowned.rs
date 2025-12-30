// Generated macro for GroupOpsOwned (trait)
macro_rules! DepcrateGroupOpsOwned {
() => {
// Module: crate
// Provides: {"GroupOpsOwned"}
// Dependencies: {}
# [doc = " A helper trait for references with a group operation."] pub trait GroupOpsOwned < Rhs = Self , Output = Self > : for < 'r > GroupOps < & 'r Rhs , Output > { }
};
}
