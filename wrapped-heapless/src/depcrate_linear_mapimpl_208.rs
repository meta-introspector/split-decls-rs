// Generated macro for impl_208 (impl)
macro_rules! Depcrate_linear_mapimpl_208 {
() => {
// Module: crate::linear_map
// Provides: {"impl_208"}
// Dependencies: {}
impl < K , V , Q , S : LinearMapStorage < K , V > + ? Sized > ops :: Index < & '_ Q > for LinearMapInner < K , V , S > where K : Borrow < Q > + Eq , Q : Eq + ? Sized , { type Output = V ; fn index (& self , key : & Q) -> & V { self . get (key) . expect ("no entry found for key") } }
};
}
