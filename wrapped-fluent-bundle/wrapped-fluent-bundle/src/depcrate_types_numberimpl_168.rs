// Generated macro for impl_168 (impl)
macro_rules! Depcrate_types_numberimpl_168 {
() => {
// Module: crate::types::number
// Provides: {"impl_168"}
// Dependencies: {}
impl From < & str > for FluentNumberType { fn from (input : & str) -> Self { match input { "cardinal" => Self :: Cardinal , "ordinal" => Self :: Ordinal , _ => Self :: default () , } } }
};
}
