// Generated macro for impl_574 (impl)
macro_rules! Depcrate_errorimpl_574 {
() => {
// Module: crate::error
// Provides: {"impl_574"}
// Dependencies: {}
impl core :: error :: Error for EncodeError { fn source (& self) -> Option < & (dyn core :: error :: Error + 'static) > { match self { Self :: RefCellAlreadyBorrowed { inner , .. } => Some (inner) , # [cfg (feature = "std")] Self :: Io { inner , .. } => Some (inner) , # [cfg (feature = "std")] Self :: InvalidSystemTime { inner , .. } => Some (inner) , _ => None , } } }
};
}
