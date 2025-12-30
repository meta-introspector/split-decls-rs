// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
impl Worker { fn new (id : usize , receiver : Arc < Mutex < mpsc :: Receiver < Job > > >) -> Worker { let thread = thread :: spawn (move | | { loop { let job = receiver . lock () . unwrap () . recv () . unwrap () ; println ! ("Worker {id} got a job; executing.") ; job () ; } }) ; Worker { id , thread } } }
};
}
