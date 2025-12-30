// Generated macro for DEFAULT_MAX_CHAIN_DEPTH (const)
macro_rules! Depcrate_policyDEFAULT_MAX_CHAIN_DEPTH {
() => {
// Module: crate::policy
// Provides: {"DEFAULT_MAX_CHAIN_DEPTH"}
// Dependencies: {}
# [doc = " A default reasonable maximum chain depth."] # [doc = ""] # [doc = " This depth was chosen to balance between common validation lengths"] # [doc = " (chains in the Web PKI are ordinarily no longer than 2 or 3 intermediates"] # [doc = " in the longest cases) and support for pathological cases."] # [doc = ""] # [doc = " Relatively little prior art for selecting a default depth exists;"] # [doc = " OpenSSL defaults to a limit of 100, which is far more permissive than"] # [doc = " necessary."] const DEFAULT_MAX_CHAIN_DEPTH : u8 = 8 ;
};
}
