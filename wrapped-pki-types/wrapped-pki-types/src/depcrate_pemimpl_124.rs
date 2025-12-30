// Generated macro for impl_124 (impl)
macro_rules! Depcrate_pemimpl_124 {
() => {
// Module: crate::pem
// Provides: {"impl_124"}
// Dependencies: {}
# [cfg (feature = "std")] impl < R : io :: BufRead , T : PemObject > Iterator for ReadIter < R , T > { type Item = Result < T , Error > ; fn next (& mut self) -> Option < Self :: Item > { loop { self . b64_buf . clear () ; return match from_buf_inner (& mut self . rd , & mut self . line , & mut self . b64_buf) { Ok (Some ((sec , item))) => match T :: from_pem (sec , item) { Some (res) => Some (Ok (res)) , None => continue , } , Ok (None) => return None , Err (err) => Some (Err (err)) , } ; } } }
};
}
