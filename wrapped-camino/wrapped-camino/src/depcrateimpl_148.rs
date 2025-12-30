// Generated macro for impl_148 (impl)
macro_rules! Depcrateimpl_148 {
() => {
// Module: crate
// Provides: {"impl_148"}
// Dependencies: {}
impl Hash for Utf8Path { fn hash < H : Hasher > (& self , state : & mut H) { for component in self . components () { component . hash (state) } } }
};
}
