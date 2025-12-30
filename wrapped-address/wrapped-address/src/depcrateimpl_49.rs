// Generated macro for impl_49 (impl)
macro_rules! Depcrateimpl_49 {
() => {
// Module: crate
// Provides: {"impl_49"}
// Dependencies: {}
# [doc = " Custom impl of Hash for Address."] # [doc = ""] # [doc = " This allows us to skip hashing the length of the address"] # [doc = " which is always the same anyway."] impl Hash for Address { fn hash < H : Hasher > (& self , state : & mut H) { state . write (self . as_array ()) ; } }
};
}
