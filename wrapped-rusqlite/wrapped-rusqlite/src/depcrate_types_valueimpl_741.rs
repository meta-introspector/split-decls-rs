// Generated macro for impl_741 (impl)
macro_rules! Depcrate_types_valueimpl_741 {
() => {
// Module: crate::types::value
// Provides: {"impl_741"}
// Dependencies: {}
impl Value { # [doc = " Returns SQLite fundamental datatype."] # [inline] # [must_use] pub fn data_type (& self) -> Type { match * self { Self :: Null => Type :: Null , Self :: Integer (_) => Type :: Integer , Self :: Real (_) => Type :: Real , Self :: Text (_) => Type :: Text , Self :: Blob (_) => Type :: Blob , } } }
};
}
