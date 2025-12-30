// Generated macro for promotable_odd_to_vec (function)
macro_rules! Depcrate_bytespromotable_odd_to_vec {
() => {
// Module: crate::bytes
// Provides: {"promotable_odd_to_vec"}
// Dependencies: {}
unsafe fn promotable_odd_to_vec (data : & AtomicPtr < () > , ptr : * const u8 , len : usize) -> Vec < u8 > { promotable_to_vec (data , ptr , len , | shared | shared . cast ()) }
};
}
