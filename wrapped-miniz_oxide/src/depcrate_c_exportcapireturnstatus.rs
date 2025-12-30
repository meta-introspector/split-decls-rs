// Generated macro for CAPIReturnStatus (enum)
macro_rules! Depcrate_c_exportCAPIReturnStatus {
() => {
// Module: crate::c_export
// Provides: {"CAPIReturnStatus"}
// Dependencies: {}
# [allow (bad_style)] # [repr (C)] # [derive (PartialEq , Eq)] pub enum CAPIReturnStatus { MZ_PARAM_ERROR = - 10000 , MZ_VERSION_ERROR = - 6 , MZ_BUF_ERROR = - 5 , MZ_MEM_ERROR = - 4 , MZ_DATA_ERROR = - 3 , MZ_STREAM_ERROR = - 2 , MZ_ERRNO = - 1 , MZ_OK = 0 , MZ_STREAM_END = 1 , MZ_NEED_DICT = 2 , }
};
}
