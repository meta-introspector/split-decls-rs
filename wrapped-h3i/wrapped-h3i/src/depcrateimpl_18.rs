// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl StreamIdAllocator { pub fn take_next_id (& mut self) -> u64 { let old = self . id ; self . id += 4 ; old } pub fn peek_next_id (& mut self) -> u64 { self . id } }
};
}
