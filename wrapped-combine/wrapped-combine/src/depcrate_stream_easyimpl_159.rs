// Generated macro for impl_159 (impl)
macro_rules! Depcrate_stream_easyimpl_159 {
() => {
// Module: crate::stream::easy
// Provides: {"impl_159"}
// Dependencies: {}
impl < T , R > Info < T , R > { pub fn map_token < F , U > (self , f : F) -> Info < U , R > where F : FnOnce (T) -> U , { use self :: Info :: * ; match self { Token (t) => Token (f (t)) , Range (r) => Range (r) , Owned (s) => Owned (s) , Static (x) => Static (x) , } } pub fn map_range < F , S > (self , f : F) -> Info < T , S > where F : FnOnce (R) -> S , { use self :: Info :: * ; match self { Token (t) => Token (t) , Range (r) => Range (f (r)) , Owned (s) => Owned (s) , Static (x) => Static (x) , } } }
};
}
