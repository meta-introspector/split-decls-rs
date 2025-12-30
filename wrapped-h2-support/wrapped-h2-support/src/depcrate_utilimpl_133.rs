// Generated macro for impl_133 (impl)
macro_rules! Depcrate_utilimpl_133 {
() => {
// Module: crate::util
// Provides: {"impl_133"}
// Dependencies: {}
impl Future for WaitForCapacity { type Output = h2 :: SendStream < Bytes > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { loop { let _ = ready ! (self . stream () . poll_capacity (cx)) . unwrap () ; let act = self . stream () . capacity () ; assert_ne ! (act , 0) ; if act >= self . target { return Poll :: Ready (self . stream . take () . unwrap ()) ; } } } }
};
}
