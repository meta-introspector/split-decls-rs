// Generated macro for impl_58 (impl)
macro_rules! Depcrate_ordered_mapimpl_58 {
() => {
// Module: crate::ordered_map
// Provides: {"impl_58"}
// Dependencies: {}
impl < 'a , K , V , T : ? Sized > Index < & 'a T > for OrderedMap < K , V > where T : Eq + PhfHash , K : PhfBorrow < T > , { type Output = V ; fn index (& self , k : & 'a T) -> & V { self . get (k) . expect ("invalid key") } }
};
}
