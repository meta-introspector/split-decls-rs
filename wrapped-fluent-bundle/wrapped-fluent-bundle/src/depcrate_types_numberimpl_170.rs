// Generated macro for impl_170 (impl)
macro_rules! Depcrate_types_numberimpl_170 {
() => {
// Module: crate::types::number
// Provides: {"impl_170"}
// Dependencies: {}
impl From < & str > for FluentNumberStyle { fn from (input : & str) -> Self { match input { "decimal" => Self :: Decimal , "currency" => Self :: Currency , "percent" => Self :: Percent , _ => Self :: default () , } } }
};
}
