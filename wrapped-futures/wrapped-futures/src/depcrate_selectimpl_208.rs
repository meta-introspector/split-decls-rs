// Generated macro for impl_208 (impl)
macro_rules! Depcrate_selectimpl_208 {
() => {
// Module: crate::select
// Provides: {"impl_208"}
// Dependencies: {}
impl < A , B > Future for Select < A , B > where A : Future , B : Future < Item = A :: Item , Error = A :: Error > , { type Item = (A :: Item , SelectNext < A , B >) ; type Error = (A :: Error , SelectNext < A , B >) ; fn poll (& mut self , task : & mut Task) -> Poll < Self :: Item , Self :: Error > { let (ret , is_a) = match self . inner { Some ((ref mut a , ref mut b)) => { match a . poll (task) { Poll :: Ok (a) => (Ok (a) , true) , Poll :: Err (a) => (Err (a) , true) , Poll :: NotReady => (try_poll ! (b . poll (task)) , false) , } } None => panic ! ("cannot poll select twice") , } ; let (a , b) = self . inner . take () . unwrap () ; let next = if is_a { OneOf :: B (b) } else { OneOf :: A (a) } ; let next = SelectNext { inner : next } ; match ret { Ok (a) => Poll :: Ok ((a , next)) , Err (e) => Poll :: Err ((e , next)) , } } fn schedule (& mut self , task : & mut Task) { match self . inner { Some ((ref mut a , ref mut b)) => { a . schedule (task) ; b . schedule (task) ; } None => task . notify () , } } fn tailcall (& mut self) -> Option < Box < Future < Item = Self :: Item , Error = Self :: Error > > > { if let Some ((ref mut a , ref mut b)) = self . inner { a . collapse () ; b . collapse () ; } None } }
};
}
