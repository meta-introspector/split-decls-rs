// Generated macro for WORD_COPY_THRESHOLD (const)
macro_rules! Depcrate_mem_implsWORD_COPY_THRESHOLD {
() => {
// Module: crate::mem::impls
// Provides: {"WORD_COPY_THRESHOLD"}
// Dependencies: {}
const WORD_COPY_THRESHOLD : usize = if 2 * WORD_SIZE > 16 { 2 * WORD_SIZE } else { 16 } ;
};
}
