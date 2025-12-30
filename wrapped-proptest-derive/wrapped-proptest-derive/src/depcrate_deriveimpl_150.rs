// Generated macro for impl_150 (impl)
macro_rules! Depcrate_deriveimpl_150 {
() => {
// Module: crate::derive
// Provides: {"impl_150"}
// Dependencies: {}
impl PartsAcc < (u32 , Ctor) > { # [doc = " Finishes off the accumulator by returning the parts needed for"] # [doc = " deriving. The resultant strategy is one that randomly picks"] # [doc = " one of the parts based on the relative weights in the `u32`."] fn finish (self , ctx : Ctx) -> ImplParts { let (params , count) = self . params . consume () ; let (strat , ctor) = self . strats . finish (ctx) ; (params , strat , extract_all (ctor , count , FromReg :: Top)) } }
};
}
