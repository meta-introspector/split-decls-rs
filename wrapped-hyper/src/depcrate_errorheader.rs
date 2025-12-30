// Generated macro for Header (enum)
macro_rules! Depcrate_errorHeader {
() => {
// Module: crate::error
// Provides: {"Header"}
// Dependencies: {}
# [derive (Debug)] # [cfg (feature = "http1")] pub (super) enum Header { Token , # [cfg (any (feature = "client" , feature = "server"))] ContentLengthInvalid , # [cfg (feature = "server")] TransferEncodingInvalid , # [cfg (any (feature = "client" , feature = "server"))] TransferEncodingUnexpected , }
};
}
