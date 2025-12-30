// Generated macro for impl_41 (impl)
macro_rules! Depcrate_simpleimpl_41 {
() => {
// Module: crate::simple
// Provides: {"impl_41"}
// Dependencies: {}
impl < D : Digest + BlockSizeUser > FixedOutput for SimpleHmac < D > { fn finalize_into (self , out : & mut Output < Self >) { let mut h = D :: new () ; h . update (& self . opad_key) ; h . update (self . digest . finalize ()) ; h . finalize_into (out) ; } }
};
}
