// Generated macro for impl_255 (impl)
macro_rules! Depcrateimpl_255 {
() => {
// Module: crate
// Provides: {"impl_255"}
// Dependencies: {}
impl < 'a , K : 'a + Eq + Hash , V : 'a , S : BuildHasher + Clone , Q > Sub < & Q > for & 'a DashMap < K , V , S > where Q : Hash + Equivalent < K > + ? Sized , { type Output = Option < (K , V) > ; fn sub (self , key : & Q) -> Self :: Output { self . remove (key) } }
};
}
