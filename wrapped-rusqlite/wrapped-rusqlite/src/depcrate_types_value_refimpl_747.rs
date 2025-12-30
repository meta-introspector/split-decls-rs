// Generated macro for impl_747 (impl)
macro_rules! Depcrate_types_value_refimpl_747 {
() => {
// Module: crate::types::value_ref
// Provides: {"impl_747"}
// Dependencies: {}
impl ValueRef < '_ > { # [doc = " Returns SQLite fundamental datatype."] # [inline] # [must_use] pub fn data_type (& self) -> Type { match * self { ValueRef :: Null => Type :: Null , ValueRef :: Integer (_) => Type :: Integer , ValueRef :: Real (_) => Type :: Real , ValueRef :: Text (_) => Type :: Text , ValueRef :: Blob (_) => Type :: Blob , } } }
};
}
