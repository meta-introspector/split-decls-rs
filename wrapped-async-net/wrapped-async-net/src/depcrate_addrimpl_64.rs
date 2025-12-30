// Generated macro for impl_64 (impl)
macro_rules! Depcrate_addrimpl_64 {
() => {
// Module: crate::addr
// Provides: {"impl_64"}
// Dependencies: {}
impl < I : Iterator < Item = SocketAddr > + Unpin > Future for ToSocketAddrsFuture < I > { type Output = io :: Result < I > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let state = mem :: replace (& mut * self , ToSocketAddrsFuture :: Done) ; match state { ToSocketAddrsFuture :: Resolving (mut task) => { let poll = Pin :: new (& mut task) . poll (cx) ; if poll . is_pending () { * self = ToSocketAddrsFuture :: Resolving (task) ; } poll } ToSocketAddrsFuture :: Ready (res) => Poll :: Ready (res) , ToSocketAddrsFuture :: Done => panic ! ("polled a completed future") , } } }
};
}
