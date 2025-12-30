// Generated macro for impl_120 (impl)
macro_rules! Depcrate_utilsimpl_120 {
() => {
// Module: crate::utils
// Provides: {"impl_120"}
// Dependencies: {}
impl Attributes { # [inline] const fn new () -> Self { Self (0) } # [inline] # [must_use] const fn insert (mut self , attr : Attribute) -> Self { let bit = attr as u16 ; self . 0 |= 1 << bit ; self } # [inline] const fn bits (self) -> BitsIter { BitsIter (self . 0) } # [inline] fn attrs (self) -> impl Iterator < Item = Attribute > { self . bits () . map (| bit | Attribute :: MAP [bit as usize]) } # [inline] fn is_empty (self) -> bool { self . 0 == 0 } }
};
}
