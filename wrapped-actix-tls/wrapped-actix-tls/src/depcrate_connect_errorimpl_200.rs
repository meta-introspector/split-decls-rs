// Generated macro for impl_200 (impl)
macro_rules! Depcrate_connect_errorimpl_200 {
() => {
// Module: crate::connect::error
// Provides: {"impl_200"}
// Dependencies: {}
impl fmt :: Display for ConnectError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: NoRecords => f . write_str ("No DNS records found for the input") , Self :: InvalidInput => f . write_str ("Invalid input") , Self :: Unresolved => { f . write_str ("Connector received `Connect` method with unresolved host") } Self :: Resolver (_) => f . write_str ("Failed to resolve hostname") , Self :: Io (_) => f . write_str ("I/O error") , } } }
};
}
