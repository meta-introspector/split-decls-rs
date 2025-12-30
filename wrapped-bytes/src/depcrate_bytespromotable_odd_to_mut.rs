// Generated macro for promotable_odd_to_mut (function)
macro_rules! Depcrate_bytespromotable_odd_to_mut {
() => {
// Module: crate::bytes
// Provides: {"promotable_odd_to_mut"}
// Dependencies: {}
unsafe fn promotable_odd_to_mut (data : & AtomicPtr < () > , ptr : * const u8 , len : usize) -> BytesMut { promotable_to_mut (data , ptr , len , | shared | shared . cast ()) }
};
}
