// Generated macro for impl_201 (impl)
macro_rules! Depcrate_connect_errorimpl_201 {
() => {
// Module: crate::connect::error
// Provides: {"impl_201"}
// Dependencies: {}
impl Error for ConnectError { fn source (& self) -> Option < & (dyn Error + 'static) > { match self { Self :: Resolver (err) => Some (& * * err) , Self :: Io (err) => Some (err) , Self :: NoRecords | Self :: InvalidInput | Self :: Unresolved => None , } } }
};
}
