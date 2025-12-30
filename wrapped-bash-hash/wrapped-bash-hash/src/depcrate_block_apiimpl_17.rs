// Generated macro for impl_17 (impl)
macro_rules! Depcrate_block_apiimpl_17 {
() => {
// Module: crate::block_api
// Provides: {"impl_17"}
// Dependencies: {}
impl < OS : OutputSize > Default for BashHashCore < OS > { # [inline] fn default () -> Self { let mut state = [0u64 ; STATE_WORDS] ; let level = OS :: USIZE * 4 ; state [23] = (level / 4) as u64 ; Self { state , _pd : PhantomData , } } }
};
}
