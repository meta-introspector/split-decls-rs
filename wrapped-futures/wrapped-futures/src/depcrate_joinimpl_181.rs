// Generated macro for impl_181 (impl)
macro_rules! Depcrate_joinimpl_181 {
() => {
// Module: crate::join
// Provides: {"impl_181"}
// Dependencies: {}
impl < A : Future > MaybeDone < A > { fn poll (& mut self , task : & mut Task) -> Result < bool , A :: Error > { let res = match * self { MaybeDone :: NotYet (ref mut a) => a . poll (task) , MaybeDone :: Done (_) => return Ok (true) , MaybeDone :: Gone => panic ! ("cannot poll Join twice") , } ; match res { Poll :: Ok (res) => { * self = MaybeDone :: Done (res) ; Ok (true) } Poll :: Err (res) => Err (res) , Poll :: NotReady => Ok (false) , } } fn take (& mut self) -> A :: Item { match mem :: replace (self , MaybeDone :: Gone) { MaybeDone :: Done (a) => a , _ => panic ! () , } } fn collapse (& mut self) { match * self { MaybeDone :: NotYet (ref mut a) => a . collapse () , _ => { } } } }
};
}
