// Generated macro for impl_27 (impl)
macro_rules! Depcrate_block_apiimpl_27 {
() => {
// Module: crate::block_api
// Provides: {"impl_27"}
// Dependencies: {}
impl < D : EagerHash > FixedOutputCore for HmacResetCore < D > { # [inline (always)] fn finalize_fixed_core (& mut self , buffer : & mut Buffer < Self > , out : & mut Output < Self >) { let mut hash = Output :: < D :: Core > :: default () ; self . digest . finalize_fixed_core (buffer , & mut hash) ; buffer . reset () ; let mut h = self . opad_digest . clone () ; buffer . digest_blocks (& hash , | b | h . update_blocks (b)) ; h . finalize_fixed_core (buffer , out) ; } }
};
}
