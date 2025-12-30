// Generated macro for impl_251 (impl)
macro_rules! Depcrate_typesimpl_251 {
() => {
// Module: crate::types
// Provides: {"impl_251"}
// Dependencies: {}
impl FutexWaitV { pub const fn new () -> Self { Self (sys :: futex_waitv { val : 0 , uaddr : 0 , flags : 0 , __reserved : 0 , }) } pub const fn val (mut self , val : u64) -> Self { self . 0 . val = val ; self } pub const fn uaddr (mut self , uaddr : u64) -> Self { self . 0 . uaddr = uaddr ; self } pub const fn flags (mut self , flags : u32) -> Self { self . 0 . flags = flags ; self } }
};
}
