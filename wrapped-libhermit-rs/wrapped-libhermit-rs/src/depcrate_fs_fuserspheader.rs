// Generated macro for RspHeader (struct)
macro_rules! Depcrate_fs_fuseRspHeader {
() => {
// Module: crate::fs::fuse
// Provides: {"RspHeader"}
// Dependencies: {}
# [repr (C)] # [derive (Debug)] pub (crate) struct RspHeader < O : ops :: Op , H = < O as ops :: Op > :: OutStruct > { pub out_header : fuse_out_header , op_header : H , _phantom : PhantomData < O :: OutStruct > , }
};
}
