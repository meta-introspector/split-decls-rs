// Generated macro for impl_931 (impl)
macro_rules! Depcrateimpl_931 {
() => {
// Module: crate
// Provides: {"impl_931"}
// Dependencies: {}
impl Hash for ConstScalar { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { core :: mem :: discriminant (self) . hash (state) ; if let ConstScalar :: Bytes (b , _) = self { b . hash (state) } } }
};
}
