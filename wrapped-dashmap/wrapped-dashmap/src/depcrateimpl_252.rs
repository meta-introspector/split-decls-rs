// Generated macro for impl_252 (impl)
macro_rules! Depcrateimpl_252 {
() => {
// Module: crate
// Provides: {"impl_252"}
// Dependencies: {}
impl < 'a , K : 'a + Eq + Hash , V : 'a , S : BuildHasher + Clone > Shl < (K , V) > for & 'a DashMap < K , V , S > { type Output = Option < V > ; fn shl (self , pair : (K , V)) -> Self :: Output { self . insert (pair . 0 , pair . 1) } }
};
}
