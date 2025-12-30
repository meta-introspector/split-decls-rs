// Generated macro for impl_253 (impl)
macro_rules! Depcrateimpl_253 {
() => {
// Module: crate
// Provides: {"impl_253"}
// Dependencies: {}
impl < 'a , K : 'a + Eq + Hash , V : 'a , S : BuildHasher + Clone , Q > Shr < & Q > for & 'a DashMap < K , V , S > where Q : Hash + Equivalent < K > + ? Sized , { type Output = Ref < 'a , K , V > ; fn shr (self , key : & Q) -> Self :: Output { self . get (key) . unwrap () } }
};
}
