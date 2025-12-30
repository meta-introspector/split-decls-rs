// Generated macro for impl_364 (impl)
macro_rules! Depcrate_value_indeximpl_364 {
() => {
// Module: crate::value::index
// Provides: {"impl_364"}
// Dependencies: {}
impl Index for String { fn index_into < 'v > (& self , v : & 'v Value) -> Option < & 'v Value > { self [..] . index_into (v) } fn index_into_mut < 'v > (& self , v : & 'v mut Value) -> Option < & 'v mut Value > { self [..] . index_into_mut (v) } fn index_or_insert < 'v > (& self , v : & 'v mut Value) -> & 'v mut Value { self [..] . index_or_insert (v) } }
};
}
