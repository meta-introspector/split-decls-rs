// Generated macro for impl_102 (impl)
macro_rules! Depcrate_inferimpl_102 {
() => {
// Module: crate::infer
// Provides: {"impl_102"}
// Dependencies: {}
impl AutoBorrow { fn mutability (self) -> Mutability { match self { AutoBorrow :: Ref (mutbl) => mutbl . into () , AutoBorrow :: RawPtr (mutbl) => mutbl , } } }
};
}
