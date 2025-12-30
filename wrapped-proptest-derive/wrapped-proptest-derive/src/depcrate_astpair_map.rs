// Generated macro for pair_map (function)
macro_rules! Depcrate_astpair_map {
() => {
// Module: crate::ast
// Provides: {"pair_map"}
// Dependencies: {}
# [doc = " The type and constructor for .prop_map:ing a set of strategies"] # [doc = " into the type we are implementing for. The closure for the"] # [doc = " `.prop_map(<closure>)` must also be given."] pub fn pair_map ((strats , ctors) : (Vec < Strategy > , Vec < Ctor >) , closure : MapClosure ,) -> StratPair { (Strategy :: Map (strats . into ()) , Ctor :: Map (ctors . into () , closure) ,) }
};
}
