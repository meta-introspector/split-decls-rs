// Generated macro for impl_302 (impl)
macro_rules! Depcrate_proto_errorimpl_302 {
() => {
// Module: crate::proto::error
// Provides: {"impl_302"}
// Dependencies: {}
impl Initiator { fn is_local (& self) -> bool { match * self { Self :: User | Self :: Library => true , Self :: Remote => false , } } pub (crate) fn is_library (& self) -> bool { matches ! (self , Self :: Library) } }
};
}
