// Generated macro for impl_691 (impl)
macro_rules! Depcrate_frame_priorityimpl_691 {
() => {
// Module: crate::frame::priority
// Provides: {"impl_691"}
// Dependencies: {}
impl Priority { pub fn load (head : Head , payload : & [u8]) -> Result < Self , Error > { let dependency = StreamDependency :: load (payload) ? ; if dependency . dependency_id () == head . stream_id () { return Err (Error :: InvalidDependencyId) ; } Ok (Priority { stream_id : head . stream_id () , dependency , }) } }
};
}
