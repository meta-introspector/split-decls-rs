// Generated macro for impl_172 (impl)
macro_rules! Depcrate_types_numberimpl_172 {
() => {
// Module: crate::types::number
// Provides: {"impl_172"}
// Dependencies: {}
impl From < & str > for FluentNumberCurrencyDisplayStyle { fn from (input : & str) -> Self { match input { "symbol" => Self :: Symbol , "code" => Self :: Code , "name" => Self :: Name , _ => Self :: default () , } } }
};
}
