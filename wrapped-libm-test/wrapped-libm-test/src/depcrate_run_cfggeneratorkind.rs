// Generated macro for GeneratorKind (enum)
macro_rules! Depcrate_run_cfgGeneratorKind {
() => {
// Module: crate::run_cfg
// Provides: {"GeneratorKind"}
// Dependencies: {}
# [doc = " The different kinds of generators that provide test input, which account for input pattern"] # [doc = " and quantity."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum GeneratorKind { # [doc = " Extremes, zeros, nonstandard numbers, etc."] EdgeCases , # [doc = " Spaced by logarithm (floats) or linear (integers)."] Spaced , # [doc = " Test inputs from an RNG."] Random , # [doc = " A provided test case list."] List , }
};
}
