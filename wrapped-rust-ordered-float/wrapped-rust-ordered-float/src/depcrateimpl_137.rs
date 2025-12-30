// Generated macro for impl_137 (impl)
macro_rules! Depcrateimpl_137 {
() => {
// Module: crate
// Provides: {"impl_137"}
// Dependencies: {}
impl TryFrom < f64 > for NotNan < f64 > { type Error = FloatIsNan ; # [inline] fn try_from (v : f64) -> Result < Self , Self :: Error > { NotNan :: new (v) } }
};
}
