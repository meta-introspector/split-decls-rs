// Generated macro for impl_173 (impl)
macro_rules! Depcrate_stream_easyimpl_173 {
() => {
// Module: crate::stream::easy
// Provides: {"impl_173"}
// Dependencies: {}
impl < T : PartialEq , R : PartialEq > PartialEq for Error < T , R > { fn eq (& self , other : & Error < T , R >) -> bool { match (self , other) { (& Error :: Unexpected (ref l) , & Error :: Unexpected (ref r)) | (& Error :: Expected (ref l) , & Error :: Expected (ref r)) | (& Error :: Message (ref l) , & Error :: Message (ref r)) => l == r , _ => false , } } }
};
}
