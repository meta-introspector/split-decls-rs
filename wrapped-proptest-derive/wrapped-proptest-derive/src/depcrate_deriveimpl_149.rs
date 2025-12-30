// Generated macro for impl_149 (impl)
macro_rules! Depcrate_deriveimpl_149 {
() => {
// Module: crate::derive
// Provides: {"impl_149"}
// Dependencies: {}
impl PartsAcc < Ctor > { # [doc = " Finishes off the accumulator by returning the parts needed for"] # [doc = " deriving. The resulting strategy is a mapping of the parts into"] # [doc = " the `Self` type."] fn finish (self , closure : MapClosure) -> ImplParts { let (params , count) = self . params . consume () ; let (strat , ctor) = self . strats . finish (closure) ; (params , strat , extract_all (ctor , count , FromReg :: Top)) } }
};
}
