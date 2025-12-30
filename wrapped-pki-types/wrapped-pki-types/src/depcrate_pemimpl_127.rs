// Generated macro for impl_127 (impl)
macro_rules! Depcrate_pemimpl_127 {
() => {
// Module: crate::pem
// Provides: {"impl_127"}
// Dependencies: {}
impl < T : PemObject > Iterator for SliceIter < '_ , T > { type Item = Result < T , Error > ; fn next (& mut self) -> Option < Self :: Item > { loop { return match self . read_section () { Ok (Some ((sec , item))) => match T :: from_pem (sec , item) { Some (res) => Some (Ok (res)) , None => continue , } , Ok (None) => return None , Err (err) => Some (Err (err)) , } ; } } }
};
}
