// Generated macro for impl_87 (impl)
macro_rules! Depcrateimpl_87 {
() => {
// Module: crate
// Provides: {"impl_87"}
// Dependencies: {}
impl < T : NumCast > NumCast for OrderedFloat < T > { # [inline] fn from < F : ToPrimitive > (n : F) -> Option < Self > { T :: from (n) . map (OrderedFloat) } }
};
}
