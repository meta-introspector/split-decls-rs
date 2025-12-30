// Generated macro for UsedBuiltinAndOptimized (type)
macro_rules! DepcrateUsedBuiltinAndOptimized {
() => {
// Module: crate
// Provides: {"UsedBuiltinAndOptimized"}
// Dependencies: {}
# [doc = " A tuple returned by the validation and processing of the parsed grammar."] # [doc = " The first element is the vector of used builtin rule names,"] # [doc = " the second element is the vector of optimized rules."] type UsedBuiltinAndOptimized < 'i > = (Vec < & 'i str > , Vec < optimizer :: OptimizedRule >) ;
};
}
