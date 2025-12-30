// Generated macro for FSTL_PIPE_PEEK (const)
macro_rules! Depcrate_ntioapiFSTL_PIPE_PEEK {
() => {
// Module: crate::ntioapi
// Provides: {"FSTL_PIPE_PEEK"}
// Dependencies: {}
pub const FSTL_PIPE_PEEK : u32 = CTL_CODE (FILE_DEVICE_NAMED_PIPE , 3 , METHOD_BUFFERED , FILE_READ_DATA) ;
};
}
