// Generated macro for impl_927 (impl)
macro_rules! Depcrate_util_shapeimpl_927 {
() => {
// Module: crate::util::shape
// Provides: {"impl_927"}
// Dependencies: {}
impl AsShape for syn :: Fields { fn as_shape (& self) -> Shape { match self { syn :: Fields :: Named (fields) => fields . as_shape () , syn :: Fields :: Unnamed (fields) => fields . as_shape () , syn :: Fields :: Unit => Shape :: Unit , } } }
};
}
