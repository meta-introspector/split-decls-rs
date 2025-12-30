// Generated macro for impl_74 (impl)
macro_rules! Depcrate_tinflimpl_74 {
() => {
// Module: crate::tinfl
// Provides: {"impl_74"}
// Dependencies: {}
impl From < TINFLStatus > for tinfl_status { fn from (status : TINFLStatus) -> tinfl_status { use self :: tinfl_status :: * ; match status { TINFLStatus :: FailedCannotMakeProgress => TINFL_STATUS_FAILED_CANNOT_MAKE_PROGRESS , TINFLStatus :: BadParam => TINFL_STATUS_BAD_PARAM , TINFLStatus :: Adler32Mismatch => TINFL_STATUS_ADLER32_MISMATCH , TINFLStatus :: Failed => TINFL_STATUS_FAILED , TINFLStatus :: Done => TINFL_STATUS_DONE , TINFLStatus :: NeedsMoreInput => TINFL_STATUS_NEEDS_MORE_INPUT , TINFLStatus :: HasMoreOutput => TINFL_STATUS_HAS_MORE_OUTPUT , } } }
};
}
