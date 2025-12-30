// Generated macro for impl_449 (impl)
macro_rules! Depcrate_numeric_literalimpl_449 {
() => {
// Module: crate::numeric_literal
// Provides: {"impl_449"}
// Dependencies: {}
impl Radix { # [doc = " Returns a reasonable digit group size for this radix."] # [must_use] fn suggest_grouping (self) -> usize { match self { Self :: Binary | Self :: Hexadecimal => 4 , Self :: Octal | Self :: Decimal => 3 , } } }
};
}
