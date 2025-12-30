// Generated macro for impl_54 (impl)
macro_rules! Depcrate_simple_resetimpl_54 {
() => {
// Module: crate::simple_reset
// Provides: {"impl_54"}
// Dependencies: {}
impl < D : Digest + BlockSizeUser > FixedOutput for SimpleHmacReset < D > { fn finalize_into (self , out : & mut Output < Self >) { let mut h = D :: new () ; h . update (& self . opad_key) ; h . update (self . digest . finalize ()) ; h . finalize_into (out) ; } }
};
}
