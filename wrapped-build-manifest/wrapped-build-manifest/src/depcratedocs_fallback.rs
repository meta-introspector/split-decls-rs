// Generated macro for DOCS_FALLBACK (static)
macro_rules! DepcrateDOCS_FALLBACK {
() => {
// Module: crate
// Provides: {"DOCS_FALLBACK"}
// Dependencies: {}
# [doc = " This allows the manifest to contain rust-docs for hosts that don't build"] # [doc = " docs."] # [doc = ""] # [doc = " Tuples of `(host_partial, host_instead)`. If the host does not have the"] # [doc = " rust-docs component available, then if the host name contains"] # [doc = " `host_partial`, it will use the docs from `host_instead` instead."] # [doc = ""] # [doc = " The order here matters, more specific entries should be first."] static DOCS_FALLBACK : & [(& str , & str)] = & [("-apple-" , "aarch64-apple-darwin") , ("aarch64" , "aarch64-unknown-linux-gnu") , ("arm-" , "aarch64-unknown-linux-gnu") , ("" , "x86_64-unknown-linux-gnu") ,] ;
};
}
