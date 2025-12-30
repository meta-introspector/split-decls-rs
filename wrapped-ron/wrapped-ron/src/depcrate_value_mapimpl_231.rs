// Generated macro for impl_231 (impl)
macro_rules! Depcrate_value_mapimpl_231 {
() => {
// Module: crate::value::map
// Provides: {"impl_231"}
// Dependencies: {}
impl Index < & Value > for Map { type Output = Value ; # [allow (clippy :: expect_used)] fn index (& self , index : & Value) -> & Self :: Output { self . get (index) . expect ("no entry found for key") } }
};
}
