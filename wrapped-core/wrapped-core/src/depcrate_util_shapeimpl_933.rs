// Generated macro for impl_933 (impl)
macro_rules! Depcrate_util_shapeimpl_933 {
() => {
// Module: crate::util::shape
// Provides: {"impl_933"}
// Dependencies: {}
impl Shape { pub fn description (& self) -> & 'static str { match self { Shape :: Named => "named fields" , Shape :: Tuple => "unnamed fields" , Shape :: Unit => "no fields" , Shape :: Newtype => "one unnamed field" , } } }
};
}
