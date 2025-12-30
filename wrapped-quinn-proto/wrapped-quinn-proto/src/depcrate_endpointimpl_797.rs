// Generated macro for impl_797 (impl)
macro_rules! Depcrate_endpointimpl_797 {
() => {
// Module: crate::endpoint
// Provides: {"impl_797"}
// Dependencies: {}
impl Index < ConnectionHandle > for Slab < ConnectionMeta > { type Output = ConnectionMeta ; fn index (& self , ch : ConnectionHandle) -> & ConnectionMeta { & self [ch . 0] } }
};
}
