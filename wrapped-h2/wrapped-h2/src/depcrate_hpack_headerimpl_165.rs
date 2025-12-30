// Generated macro for impl_165 (impl)
macro_rules! Depcrate_hpack_headerimpl_165 {
() => {
// Module: crate::hpack::header
// Provides: {"impl_165"}
// Dependencies: {}
impl Header < Option < HeaderName > > { pub fn reify (self) -> Result < Header , HeaderValue > { use self :: Header :: * ; Ok (match self { Field { name : Some (n) , value , } => Field { name : n , value } , Field { name : None , value } => return Err (value) , Authority (v) => Authority (v) , Method (v) => Method (v) , Scheme (v) => Scheme (v) , Path (v) => Path (v) , Protocol (v) => Protocol (v) , Status (v) => Status (v) , }) } }
};
}
