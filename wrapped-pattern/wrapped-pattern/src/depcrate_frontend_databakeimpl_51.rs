// Generated macro for impl_51 (impl)
macro_rules! Depcrate_frontend_databakeimpl_51 {
() => {
// Module: crate::frontend::databake
// Provides: {"impl_51"}
// Dependencies: {}
impl < B > BakeSize for & Pattern < B > where B : PatternBackend , for < 'b > & 'b B :: Store : BakeSize , { fn borrows_size (& self) -> usize { let s : & B :: Store = & self . store ; s . borrows_size () } }
};
}
