// Generated macro for impl_29 (impl)
macro_rules! Depcrateimpl_29 {
() => {
// Module: crate
// Provides: {"impl_29"}
// Dependencies: {}
impl Stream for ReadDir { type Item = io :: Result < DirEntry > ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { loop { match & mut self . 0 { State :: Idle (opt) => { let mut inner = opt . take () . unwrap () ; self . 0 = State :: Busy (unblock (move | | { let next = inner . next () ; (inner , next) })) ; } State :: Busy (task) => { let (inner , opt) = ready ! (task . poll (cx)) ; self . 0 = State :: Idle (Some (inner)) ; return Poll :: Ready (opt . map (| res | res . map (| inner | DirEntry (Arc :: new (inner))))) ; } } } } }
};
}
