// Generated macro for impl_57 (impl)
macro_rules! Depcrate_tdefimpl_57 {
() => {
// Module: crate::tdef
// Provides: {"impl_57"}
// Dependencies: {}
impl From < TDEFLStatus > for tdefl_status { fn from (status : TDEFLStatus) -> tdefl_status { use self :: tdefl_status :: * ; match status { TDEFLStatus :: BadParam => TDEFL_STATUS_BAD_PARAM , TDEFLStatus :: PutBufFailed => TDEFL_STATUS_PUT_BUF_FAILED , TDEFLStatus :: Okay => TDEFL_STATUS_OKAY , TDEFLStatus :: Done => TDEFL_STATUS_DONE , } } }
};
}
