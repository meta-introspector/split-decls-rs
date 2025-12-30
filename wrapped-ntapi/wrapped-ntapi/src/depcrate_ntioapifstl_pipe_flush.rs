// Generated macro for FSTL_PIPE_FLUSH (const)
macro_rules! Depcrate_ntioapiFSTL_PIPE_FLUSH {
() => {
// Module: crate::ntioapi
// Provides: {"FSTL_PIPE_FLUSH"}
// Dependencies: {}
pub const FSTL_PIPE_FLUSH : u32 = CTL_CODE (FILE_DEVICE_NAMED_PIPE , 16 , METHOD_BUFFERED , FILE_WRITE_DATA) ;
};
}
