// Generated macro for impl_27 (impl)
macro_rules! Depcrateimpl_27 {
() => {
// Module: crate
// Provides: {"impl_27"}
// Dependencies: {}
impl < B : BitBlock > hash :: Hash for BitSet < B > { fn hash < H : hash :: Hasher > (& self , state : & mut H) { for pos in self { pos . hash (state) ; } } }
};
}
