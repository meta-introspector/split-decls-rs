// Generated macro for impl_214 (impl)
macro_rules! Depcrateimpl_214 {
() => {
// Module: crate
// Provides: {"impl_214"}
// Dependencies: {}
impl fmt :: Debug for ChunkState { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("ChunkState") . field ("count" , & self . count ()) . field ("chunk_counter" , & self . chunk_counter) . field ("flags" , & self . flags) . field ("platform" , & self . platform) . finish () } }
};
}
