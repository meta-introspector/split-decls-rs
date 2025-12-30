// Generated macro for impl_57 (impl)
macro_rules! Depcrate_simple_resetimpl_57 {
() => {
// Module: crate::simple_reset
// Provides: {"impl_57"}
// Dependencies: {}
impl < D : Digest + BlockSizeUser + FixedOutputReset > FixedOutputReset for SimpleHmacReset < D > { fn finalize_into_reset (& mut self , out : & mut Output < Self >) { let mut h = D :: new () ; Update :: update (& mut h , & self . opad_key) ; Update :: update (& mut h , & self . digest . finalize_reset ()) ; Update :: update (& mut self . digest , & self . ipad_key) ; Digest :: finalize_into (h , out) ; } }
};
}
