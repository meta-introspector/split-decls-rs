// Generated macro for impl_39 (impl)
macro_rules! Depcrateimpl_39 {
() => {
// Module: crate
// Provides: {"impl_39"}
// Dependencies: {}
impl < K : Hash + Eq , V , S : BuildHasher > fmt :: Debug for LruCache < K , V , S > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("LruCache") . field ("len" , & self . len ()) . field ("cap" , & self . cap ()) . finish () } }
};
}
