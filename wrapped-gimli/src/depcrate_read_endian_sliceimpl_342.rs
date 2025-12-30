// Generated macro for impl_342 (impl)
macro_rules! Depcrate_read_endian_sliceimpl_342 {
() => {
// Module: crate::read::endian_slice
// Provides: {"impl_342"}
// Dependencies: {}
impl < 'input , Endian : Endianity > fmt :: Debug for EndianSlice < 'input , Endian > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> core :: result :: Result < () , fmt :: Error > { fmt . debug_tuple ("EndianSlice") . field (& self . endian) . field (& DebugBytes (self . slice)) . finish () } }
};
}
