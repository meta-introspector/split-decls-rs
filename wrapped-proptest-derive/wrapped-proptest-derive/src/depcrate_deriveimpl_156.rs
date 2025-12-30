// Generated macro for impl_156 (impl)
macro_rules! Depcrate_deriveimpl_156 {
() => {
// Module: crate::derive
// Provides: {"impl_156"}
// Dependencies: {}
impl StratAcc < (u32 , Ctor) > { # [doc = " Finishes off the accumulator by returning a union of the"] # [doc = " strategies where the resultant strategy randomly picks"] # [doc = " one of the summands based on the relative weights provided."] fn finish (self , ctx : Ctx) -> StratPair { if self . ctors . iter () . map (| & (w , _) | w) . try_fold (0u32 , | acc , w | acc . checked_add (w)) . is_none () { error :: weight_overflowing (ctx) } pair_oneof (self . consume ()) } }
};
}
