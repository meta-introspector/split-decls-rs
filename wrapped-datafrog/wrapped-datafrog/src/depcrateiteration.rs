// Generated macro for Iteration (struct)
macro_rules! DepcrateIteration {
() => {
// Module: crate
// Provides: {"Iteration"}
// Dependencies: {}
# [doc = " An iterative context for recursive evaluation."] # [doc = ""] # [doc = " An `Iteration` tracks monotonic variables, and monitors their progress."] # [doc = " It can inform the user if they have ceased changing, at which point the"] # [doc = " computation should be done."] pub struct Iteration { variables : Vec < Box < dyn VariableTrait > > , }
};
}
