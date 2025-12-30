// Generated macro for owned_to_mut (function)
macro_rules! Depcrate_bytesowned_to_mut {
() => {
// Module: crate::bytes
// Provides: {"owned_to_mut"}
// Dependencies: {}
unsafe fn owned_to_mut < T > (data : & AtomicPtr < () > , ptr : * const u8 , len : usize) -> BytesMut { BytesMut :: from_vec (owned_to_vec :: < T > (data , ptr , len)) }
};
}
