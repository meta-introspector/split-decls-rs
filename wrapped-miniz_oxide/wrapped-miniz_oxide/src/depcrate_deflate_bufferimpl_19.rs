// Generated macro for impl_19 (impl)
macro_rules! Depcrate_deflate_bufferimpl_19 {
() => {
// Module: crate::deflate::buffer
// Provides: {"impl_19"}
// Dependencies: {}
impl Default for HashBuffers { fn default () -> HashBuffers { HashBuffers { dict : vec ! [0 ; LZ_DICT_FULL_SIZE] . into_boxed_slice () . try_into () . unwrap () , next : vec ! [0 ; LZ_DICT_SIZE] . into_boxed_slice () . try_into () . unwrap () , hash : vec ! [0 ; LZ_DICT_SIZE] . into_boxed_slice () . try_into () . unwrap () , } } }
};
}
