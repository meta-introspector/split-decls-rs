// Generated macro for impl_256 (impl)
macro_rules! Depcrateimpl_256 {
() => {
// Module: crate
// Provides: {"impl_256"}
// Dependencies: {}
impl < 'a , K : 'a + Eq + Hash , V : 'a , S : BuildHasher + Clone , Q > BitAnd < & Q > for & 'a DashMap < K , V , S > where Q : Hash + Equivalent < K > + ? Sized , { type Output = bool ; fn bitand (self , key : & Q) -> Self :: Output { self . contains_key (key) } }
};
}
