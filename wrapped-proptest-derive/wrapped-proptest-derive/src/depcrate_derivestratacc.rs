// Generated macro for StratAcc (struct)
macro_rules! Depcrate_deriveStratAcc {
() => {
// Module: crate::derive
// Provides: {"StratAcc"}
// Dependencies: {}
# [doc = " Accumulator of a sequence of strategies (both type and constructor)."] struct StratAcc < C > { # [doc = " The type half of the accumulator:"] types : Vec < Strategy > , # [doc = " The constructors (Rust expression that makes the strategy) half:"] ctors : Vec < C > , }
};
}
