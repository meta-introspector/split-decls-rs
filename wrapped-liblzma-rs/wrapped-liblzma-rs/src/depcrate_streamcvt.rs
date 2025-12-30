// Generated macro for cvt (function)
macro_rules! Depcrate_streamcvt {
() => {
// Module: crate::stream
// Provides: {"cvt"}
// Dependencies: {}
fn cvt (rc : liblzma_sys :: lzma_ret) -> Result < Status , Error > { match rc { liblzma_sys :: LZMA_OK => Ok (Status :: Ok) , liblzma_sys :: LZMA_STREAM_END => Ok (Status :: StreamEnd) , liblzma_sys :: LZMA_NO_CHECK => Err (Error :: NoCheck) , liblzma_sys :: LZMA_UNSUPPORTED_CHECK => Err (Error :: UnsupportedCheck) , liblzma_sys :: LZMA_GET_CHECK => Ok (Status :: GetCheck) , liblzma_sys :: LZMA_MEM_ERROR => Err (Error :: Mem) , liblzma_sys :: LZMA_MEMLIMIT_ERROR => Err (Error :: MemLimit) , liblzma_sys :: LZMA_FORMAT_ERROR => Err (Error :: Format) , liblzma_sys :: LZMA_OPTIONS_ERROR => Err (Error :: Options) , liblzma_sys :: LZMA_DATA_ERROR => Err (Error :: Data) , liblzma_sys :: LZMA_BUF_ERROR => Ok (Status :: MemNeeded) , liblzma_sys :: LZMA_PROG_ERROR => Err (Error :: Program) , c => panic ! ("unknown return code: {}" , c) , } }
};
}
