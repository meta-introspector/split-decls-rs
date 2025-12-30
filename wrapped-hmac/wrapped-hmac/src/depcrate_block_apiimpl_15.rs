// Generated macro for impl_15 (impl)
macro_rules! Depcrate_block_apiimpl_15 {
() => {
// Module: crate::block_api
// Provides: {"impl_15"}
// Dependencies: {}
impl < D : EagerHash > FixedOutputCore for HmacCore < D > { # [inline (always)] fn finalize_fixed_core (& mut self , buffer : & mut Buffer < Self > , out : & mut Output < Self >) { let mut hash = Output :: < D :: Core > :: default () ; self . digest . finalize_fixed_core (buffer , & mut hash) ; buffer . reset () ; let h = & mut self . opad_digest ; buffer . digest_blocks (& hash , | b | h . update_blocks (b)) ; h . finalize_fixed_core (buffer , out) ; } }
};
}
