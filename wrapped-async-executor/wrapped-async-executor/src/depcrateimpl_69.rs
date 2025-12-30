// Generated macro for impl_69 (impl)
macro_rules! Depcrateimpl_69 {
() => {
// Module: crate
// Provides: {"impl_69"}
// Dependencies: {}
impl < Fut : Future , Cleanup : FnMut () > Future for AsyncCallOnDrop < Fut , Cleanup > { type Output = Fut :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . project () . future . poll (cx) } }
};
}
