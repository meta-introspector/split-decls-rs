// Generated macro for shared_to_mut (function)
macro_rules! Depcrate_bytesshared_to_mut {
() => {
// Module: crate::bytes
// Provides: {"shared_to_mut"}
// Dependencies: {}
unsafe fn shared_to_mut (data : & AtomicPtr < () > , ptr : * const u8 , len : usize) -> BytesMut { shared_to_mut_impl (data . load (Ordering :: Relaxed) . cast () , ptr , len) }
};
}
