// Generated macro for impl_433 (impl)
macro_rules! Depcrate_iterimpl_433 {
() => {
// Module: crate::iter
// Provides: {"impl_433"}
// Dependencies: {}
impl < I > Iterator for LineColIterator < I > where I : Iterator < Item = io :: Result < u8 > > , { type Item = io :: Result < u8 > ; fn next (& mut self) -> Option < io :: Result < u8 > > { match self . iter . next () { None => None , Some (Ok (b'\n')) => { self . start_of_line += self . col + 1 ; self . line += 1 ; self . col = 0 ; Some (Ok (b'\n')) } Some (Ok (c)) => { self . col += 1 ; Some (Ok (c)) } Some (Err (e)) => Some (Err (e)) , } } }
};
}
