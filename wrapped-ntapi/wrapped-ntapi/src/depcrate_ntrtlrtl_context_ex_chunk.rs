// Generated macro for RTL_CONTEXT_EX_CHUNK (macro)
macro_rules! Depcrate_ntrtlRTL_CONTEXT_EX_CHUNK {
() => {
// Module: crate::ntrtl
// Provides: {"RTL_CONTEXT_EX_CHUNK"}
// Dependencies: {}
# [macro_export] macro_rules ! RTL_CONTEXT_EX_CHUNK { ($ Base : expr , $ Layout : expr , $ Chunk : ident) => { ($ Base as usize + RTL_CONTEXT_EX_OFFSET ! ($ Layout , $ Chunk) as usize) as * mut c_void } ; }
};
}
