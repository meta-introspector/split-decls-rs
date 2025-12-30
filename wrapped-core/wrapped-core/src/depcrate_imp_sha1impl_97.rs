// Generated macro for impl_97 (impl)
macro_rules! Depcrate_imp_sha1impl_97 {
() => {
// Module: crate::imp::sha1
// Provides: {"impl_97"}
// Dependencies: {}
impl core :: fmt :: Display for Digest { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { for i in self . data . iter () { write ! (f , "{i:08x}") ? ; } Ok (()) } }
};
}
