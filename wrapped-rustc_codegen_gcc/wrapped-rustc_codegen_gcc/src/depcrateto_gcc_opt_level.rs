// Generated macro for to_gcc_opt_level (function)
macro_rules! Depcrateto_gcc_opt_level {
() => {
// Module: crate
// Provides: {"to_gcc_opt_level"}
// Dependencies: {}
fn to_gcc_opt_level (optlevel : Option < OptLevel >) -> OptimizationLevel { match optlevel { None => OptimizationLevel :: None , Some (level) => match level { OptLevel :: No => OptimizationLevel :: None , OptLevel :: Less => OptimizationLevel :: Limited , OptLevel :: More => OptimizationLevel :: Standard , OptLevel :: Aggressive => OptimizationLevel :: Aggressive , OptLevel :: Size | OptLevel :: SizeMin => OptimizationLevel :: Limited , } , } }
};
}
