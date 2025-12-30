// Generated macro for impl_929 (impl)
macro_rules! Depcrate_util_shapeimpl_929 {
() => {
// Module: crate::util::shape
// Provides: {"impl_929"}
// Dependencies: {}
impl AsShape for syn :: FieldsUnnamed { fn as_shape (& self) -> Shape { if self . unnamed . len () == 1 { Shape :: Newtype } else { Shape :: Tuple } } }
};
}
