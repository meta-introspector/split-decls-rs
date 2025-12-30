// Generated macro for impl_172 (impl)
macro_rules! Depcrate_stream_easyimpl_172 {
() => {
// Module: crate::stream::easy
// Provides: {"impl_172"}
// Dependencies: {}
impl < T , R > Error < T , R > { pub fn map_token < F , U > (self , f : F) -> Error < U , R > where F : FnOnce (T) -> U , { use self :: Error :: * ; match self { Unexpected (x) => Unexpected (x . map_token (f)) , Expected (x) => Expected (x . map_token (f)) , Message (x) => Message (x . map_token (f)) , Other (x) => Other (x) , } } pub fn map_range < F , S > (self , f : F) -> Error < T , S > where F : FnOnce (R) -> S , { use self :: Error :: * ; match self { Unexpected (x) => Unexpected (x . map_range (f)) , Expected (x) => Expected (x . map_range (f)) , Message (x) => Message (x . map_range (f)) , Other (x) => Other (x) , } } }
};
}
