// Generated macro for impl_21 (impl)
macro_rules! Depcrate_mapimpl_21 {
() => {
// Module: crate::map
// Provides: {"impl_21"}
// Dependencies: {}
impl < 'a , K , V , T : ? Sized > Index < & 'a T > for Map < K , V > where T : Eq + PhfHash , K : PhfBorrow < T > , { type Output = V ; fn index (& self , k : & 'a T) -> & V { self . get (k) . expect ("invalid key") } }
};
}
