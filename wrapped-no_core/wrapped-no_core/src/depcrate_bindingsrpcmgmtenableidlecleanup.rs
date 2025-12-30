// Generated macro for RpcMgmtEnableIdleCleanup (function)
macro_rules! Depcrate_bindingsRpcMgmtEnableIdleCleanup {
() => {
// Module: crate::bindings
// Provides: {"RpcMgmtEnableIdleCleanup"}
// Dependencies: {}
# [inline] pub unsafe fn RpcMgmtEnableIdleCleanup () -> windows_result :: RPC_STATUS { windows_link :: link ! ("rpcrt4.dll" "system" fn RpcMgmtEnableIdleCleanup () -> windows_result :: RPC_STATUS) ; unsafe { RpcMgmtEnableIdleCleanup () } }
};
}
