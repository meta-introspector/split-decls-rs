// Generated macro for impl_15 (impl)
macro_rules! Depcrate_bufimpl_15 {
() => {
// Module: crate::buf
// Provides: {"impl_15"}
// Dependencies: {}
impl < const SIZE : usize > Hash for WriteBuffer < SIZE > { fn hash < H : Hasher > (& self , state : & mut H) { self . as_str () . hash (state) } }
};
}
