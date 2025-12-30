// Generated macro for CLEAR_PARSE_MESSAGE_CONTEXT_FLAG (function)
macro_rules! Depcrate_ntrtlCLEAR_PARSE_MESSAGE_CONTEXT_FLAG {
() => {
// Module: crate::ntrtl
// Provides: {"CLEAR_PARSE_MESSAGE_CONTEXT_FLAG"}
// Dependencies: {}
# [inline] pub fn CLEAR_PARSE_MESSAGE_CONTEXT_FLAG (ctx : & mut PARSE_MESSAGE_CONTEXT , flag : ULONG) -> ULONG { ctx . fFlags &= ! flag ; ctx . fFlags }
};
}
