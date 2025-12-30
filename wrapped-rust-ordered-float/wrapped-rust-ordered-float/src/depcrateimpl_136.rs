// Generated macro for impl_136 (impl)
macro_rules! Depcrateimpl_136 {
() => {
// Module: crate
// Provides: {"impl_136"}
// Dependencies: {}
impl TryFrom < f32 > for NotNan < f32 > { type Error = FloatIsNan ; # [inline] fn try_from (v : f32) -> Result < Self , Self :: Error > { NotNan :: new (v) } }
};
}
