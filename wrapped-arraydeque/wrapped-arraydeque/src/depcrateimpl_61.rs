// Generated macro for impl_61 (impl)
macro_rules! Depcrateimpl_61 {
() => {
// Module: crate
// Provides: {"impl_61"}
// Dependencies: {}
impl < T , const CAP : usize , B : Behavior > Hash for ArrayDeque < T , CAP , B > where T : Hash , { fn hash < H : Hasher > (& self , state : & mut H) { self . len () . hash (state) ; let (a , b) = self . as_slices () ; Hash :: hash_slice (a , state) ; Hash :: hash_slice (b , state) ; } }
};
}
