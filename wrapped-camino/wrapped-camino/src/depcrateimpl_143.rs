// Generated macro for impl_143 (impl)
macro_rules! Depcrateimpl_143 {
() => {
// Module: crate
// Provides: {"impl_143"}
// Dependencies: {}
impl Hash for Utf8PathBuf { # [inline] fn hash < H : Hasher > (& self , state : & mut H) { self . as_path () . hash (state) } }
};
}
