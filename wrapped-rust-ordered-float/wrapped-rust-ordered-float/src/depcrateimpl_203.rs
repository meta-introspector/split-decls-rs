// Generated macro for impl_203 (impl)
macro_rules! Depcrateimpl_203 {
() => {
// Module: crate
// Provides: {"impl_203"}
// Dependencies: {}
impl < T : FloatCore > NumCast for NotNan < T > { fn from < F : ToPrimitive > (n : F) -> Option < Self > { T :: from (n) . and_then (| n | NotNan :: new (n) . ok ()) } }
};
}
