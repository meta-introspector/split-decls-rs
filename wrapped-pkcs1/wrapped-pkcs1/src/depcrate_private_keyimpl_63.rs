// Generated macro for impl_63 (impl)
macro_rules! Depcrate_private_keyimpl_63 {
() => {
// Module: crate::private_key
// Provides: {"impl_63"}
// Dependencies: {}
# [cfg (not (feature = "alloc"))] impl EncodeValue for OtherPrimeInfos < '_ > { fn value_len (& self) -> der :: Result < Length > { Err (der :: ErrorKind :: Value { tag : Tag :: Integer } . into ()) } fn encode_value (& self , _writer : & mut impl Writer) -> der :: Result < () > { Err (der :: ErrorKind :: Value { tag : Tag :: Integer } . into ()) } }
};
}
