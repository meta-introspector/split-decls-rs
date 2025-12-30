// Generated macro for impl_863 (impl)
macro_rules! Depcrate_serverimpl_863 {
() => {
// Module: crate::server
// Provides: {"impl_863"}
// Dependencies: {}
impl < T , B : Buf > ReadPreface < T , B > { fn new (codec : Codec < T , B >) -> Self { ReadPreface { codec : Some (codec) , pos : 0 , } } fn inner_mut (& mut self) -> & mut T { self . codec . as_mut () . unwrap () . get_mut () } }
};
}
