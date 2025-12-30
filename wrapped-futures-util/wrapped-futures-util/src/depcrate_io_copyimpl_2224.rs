// Generated macro for impl_2224 (impl)
macro_rules! Depcrate_io_copyimpl_2224 {
() => {
// Module: crate::io::copy
// Provides: {"impl_2224"}
// Dependencies: {}
impl < R : AsyncRead , W : AsyncWrite + Unpin + ? Sized > Future for Copy < '_ , R , W > { type Output = io :: Result < u64 > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . project () . inner . poll (cx) } }
};
}
