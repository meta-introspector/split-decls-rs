// Generated macro for impl_697 (impl)
macro_rules! Depcrate_naiveimpl_697 {
() => {
// Module: crate::naive
// Provides: {"impl_697"}
// Dependencies: {}
impl Hash for NaiveWeek { fn hash < H : Hasher > (& self , state : & mut H) { self . first_day () . hash (state) ; } }
};
}
