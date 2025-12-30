// Generated macro for sip_impl (macro)
macro_rules! Depcratesip_impl {
() => {
// Module: crate
// Provides: {"sip_impl"}
// Dependencies: {}
macro_rules ! sip_impl ((le $ t : ty) => (impl PhfHash for $ t { # [inline] fn phf_hash < H : Hasher > (& self , state : & mut H) { self . to_le () . hash (state) ; } }) ; ($ t : ty) => (impl PhfHash for $ t { # [inline] fn phf_hash < H : Hasher > (& self , state : & mut H) { self . hash (state) ; } })) ;
};
}
