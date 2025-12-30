// Generated macro for impl_232 (impl)
macro_rules! Depcrate_value_mapimpl_232 {
() => {
// Module: crate::value::map
// Provides: {"impl_232"}
// Dependencies: {}
impl IndexMut < & Value > for Map { # [allow (clippy :: expect_used)] fn index_mut (& mut self , index : & Value) -> & mut Self :: Output { self . get_mut (index) . expect ("no entry found for key") } }
};
}
