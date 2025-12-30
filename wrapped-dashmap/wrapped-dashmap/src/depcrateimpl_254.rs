// Generated macro for impl_254 (impl)
macro_rules! Depcrateimpl_254 {
() => {
// Module: crate
// Provides: {"impl_254"}
// Dependencies: {}
impl < 'a , K : 'a + Eq + Hash , V : 'a , S : BuildHasher + Clone , Q > BitOr < & Q > for & 'a DashMap < K , V , S > where Q : Hash + Equivalent < K > + ? Sized , { type Output = RefMut < 'a , K , V > ; fn bitor (self , key : & Q) -> Self :: Output { self . get_mut (key) . unwrap () } }
};
}
