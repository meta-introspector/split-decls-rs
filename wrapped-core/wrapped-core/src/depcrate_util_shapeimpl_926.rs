// Generated macro for impl_926 (impl)
macro_rules! Depcrate_util_shapeimpl_926 {
() => {
// Module: crate::util::shape
// Provides: {"impl_926"}
// Dependencies: {}
impl < T > AsShape for ast :: Fields < T > { fn as_shape (& self) -> Shape { match self . style { ast :: Style :: Tuple if self . fields . len () == 1 => Shape :: Newtype , ast :: Style :: Tuple => Shape :: Tuple , ast :: Style :: Struct => Shape :: Named , ast :: Style :: Unit => Shape :: Unit , } } }
};
}
