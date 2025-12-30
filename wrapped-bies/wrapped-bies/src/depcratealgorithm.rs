// Generated macro for Algorithm (enum)
macro_rules! DepcrateAlgorithm {
() => {
// Module: crate
// Provides: {"Algorithm"}
// Dependencies: {}
# [derive (Clone , Copy , Debug , PartialEq , EnumIter)] pub enum Algorithm { # [doc = " Algorithm 1a: check probabilities surrounding each valid breakpoint. Switch based on the sum."] Alg1a , # [doc = " Algorithm 1b: check probabilities surrounding each valid breakpoint. Switch based on the individual max."] Alg1b , # [doc = " Algorithm 2: step forward through the matrix and pick the highest probability at each step"] Alg2a , # [doc = " Algorithm 3: exhaustively check all combinations of breakpoints to find the highest true probability"] Alg3a , }
};
}
