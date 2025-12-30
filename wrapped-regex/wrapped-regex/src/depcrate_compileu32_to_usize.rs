// Generated macro for u32_to_usize (function)
macro_rules! Depcrate_compileu32_to_usize {
() => {
// Module: crate::compile
// Provides: {"u32_to_usize"}
// Dependencies: {}
fn u32_to_usize (n : u32) -> usize { if (n as u64) > (:: std :: usize :: MAX as u64) { panic ! ("BUG: {} is too big to be pointer sized" , n) } n as usize }
};
}
