// Generated macro for impl_37 (impl)
macro_rules! Depcrateimpl_37 {
() => {
// Module: crate
// Provides: {"impl_37"}
// Dependencies: {}
impl < 'a , T : 'a + PhfHash + ? Sized > PhfHash for & 'a T { fn phf_hash < H : Hasher > (& self , state : & mut H) { (* self) . phf_hash (state) } }
};
}
