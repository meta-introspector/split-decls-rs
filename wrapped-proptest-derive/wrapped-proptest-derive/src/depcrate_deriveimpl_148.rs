// Generated macro for impl_148 (impl)
macro_rules! Depcrate_deriveimpl_148 {
() => {
// Module: crate::derive
// Provides: {"impl_148"}
// Dependencies: {}
impl < C > PartsAcc < C > { # [doc = " Constructs a new accumulator with the size"] # [doc = " passed on to the accumulator for the strategies."] fn new (size : usize) -> Self { Self { params : ParamAcc :: empty () , strats : StratAcc :: new (size) , } } # [doc = " Adds a strategy to the accumulator."] fn add_strat (self , pair : (Strategy , C)) -> Self { Self { strats : self . strats . add (pair) , params : self . params , } } # [doc = " Adds a parameter type to the accumulator and returns how many types"] # [doc = " there were before adding."] fn add_param (& mut self , ty : Type) -> usize { self . params . add (ty) } }
};
}
