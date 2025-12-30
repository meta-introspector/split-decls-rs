// Generated macro for impl_182 (impl)
macro_rules! Depcrate_inflate_coreimpl_182 {
() => {
// Module: crate::inflate::core
// Provides: {"impl_182"}
// Dependencies: {}
impl State { # [cfg (not (feature = "rustc-dep-of-std"))] const fn is_failure (self) -> bool { matches ! (self , BlockTypeUnexpected | BadCodeSizeSum | BadDistOrLiteralTableLength | BadTotalSymbols | BadZlibHeader | DistanceOutOfBounds | BadRawLength | BadCodeSizeDistPrevLookup | InvalidLitlen | InvalidDist) } # [inline] fn begin (& mut self , new_state : State) { * self = new_state ; } }
};
}
