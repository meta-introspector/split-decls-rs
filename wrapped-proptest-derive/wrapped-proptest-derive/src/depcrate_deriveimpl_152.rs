// Generated macro for impl_152 (impl)
macro_rules! Depcrate_deriveimpl_152 {
() => {
// Module: crate::derive
// Provides: {"impl_152"}
// Dependencies: {}
impl ParamAcc { # [doc = " Returns an empty accumulator."] fn empty () -> Self { Self { types : Params :: empty () , } } # [doc = " Adds a type to the accumulator and returns the type count before adding."] fn add (& mut self , ty : Type) -> usize { let var = self . types . len () ; self . types += ty ; var } # [doc = " Consumes the accumulator returning the types and the count."] fn consume (self) -> (Params , usize) { let count = self . types . len () ; (self . types , count) } }
};
}
