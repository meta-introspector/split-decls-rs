// Generated macro for BashHashCore (struct)
macro_rules! Depcrate_block_apiBashHashCore {
() => {
// Module: crate::block_api
// Provides: {"BashHashCore"}
// Dependencies: {}
# [doc = " Core `bash-hash` hasher generic over output size."] # [doc = ""] # [doc = " Specified in Section 7 of STB 34.101.77-2020."] pub struct BashHashCore < OS : OutputSize > { state : [u64 ; STATE_WORDS] , _pd : PhantomData < OS > , }
};
}
