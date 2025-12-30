// Generated macro for impl_693 (impl)
macro_rules! Depcrate_frame_priorityimpl_693 {
() => {
// Module: crate::frame::priority
// Provides: {"impl_693"}
// Dependencies: {}
impl StreamDependency { pub fn new (dependency_id : StreamId , weight : u8 , is_exclusive : bool) -> Self { StreamDependency { dependency_id , weight , is_exclusive , } } pub fn load (src : & [u8]) -> Result < Self , Error > { if src . len () != 5 { return Err (Error :: InvalidPayloadLength) ; } let (dependency_id , is_exclusive) = StreamId :: parse (& src [.. 4]) ; let weight = src [4] ; Ok (StreamDependency :: new (dependency_id , weight , is_exclusive)) } pub fn dependency_id (& self) -> StreamId { self . dependency_id } }
};
}
