// Generated macro for impl_209 (impl)
macro_rules! Depcrate_linear_mapimpl_209 {
() => {
// Module: crate::linear_map
// Provides: {"impl_209"}
// Dependencies: {}
impl < K , V , Q , S : LinearMapStorage < K , V > + ? Sized > ops :: IndexMut < & '_ Q > for LinearMapInner < K , V , S > where K : Borrow < Q > + Eq , Q : Eq + ? Sized , { fn index_mut (& mut self , key : & Q) -> & mut V { self . get_mut (key) . expect ("no entry found for key") } }
};
}
