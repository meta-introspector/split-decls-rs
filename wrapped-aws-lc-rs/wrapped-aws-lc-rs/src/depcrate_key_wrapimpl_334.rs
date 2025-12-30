// Generated macro for impl_334 (impl)
macro_rules! Depcrate_key_wrapimpl_334 {
() => {
// Module: crate::key_wrap
// Provides: {"impl_334"}
// Dependencies: {}
impl < Cipher : BlockCipher > Debug for KeyEncryptionKey < Cipher > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_struct ("KeyEncryptionKey") . field ("cipher" , & self . cipher) . finish_non_exhaustive () } }
};
}
