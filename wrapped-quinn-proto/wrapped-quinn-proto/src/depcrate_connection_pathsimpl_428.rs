// Generated macro for impl_428 (impl)
macro_rules! Depcrate_connection_pathsimpl_428 {
() => {
// Module: crate::connection::paths
// Provides: {"impl_428"}
// Dependencies: {}
impl PathResponses { pub (crate) fn push (& mut self , packet : u64 , token : u64 , remote : SocketAddr) { # [doc = " Arbitrary permissive limit to prevent abuse"] const MAX_PATH_RESPONSES : usize = 16 ; let response = PathResponse { packet , token , remote , } ; let existing = self . pending . iter_mut () . find (| x | x . remote == remote) ; if let Some (existing) = existing { if existing . packet <= packet { * existing = response ; } return ; } if self . pending . len () < MAX_PATH_RESPONSES { self . pending . push (response) ; } else { trace ! ("ignoring excessive PATH_CHALLENGE") ; } } pub (crate) fn pop_off_path (& mut self , remote : SocketAddr) -> Option < (u64 , SocketAddr) > { let response = * self . pending . last () ? ; if response . remote == remote { return None ; } self . pending . pop () ; Some ((response . token , response . remote)) } pub (crate) fn pop_on_path (& mut self , remote : SocketAddr) -> Option < u64 > { let response = * self . pending . last () ? ; if response . remote != remote { return None ; } self . pending . pop () ; Some (response . token) } pub (crate) fn is_empty (& self) -> bool { self . pending . is_empty () } }
};
}
