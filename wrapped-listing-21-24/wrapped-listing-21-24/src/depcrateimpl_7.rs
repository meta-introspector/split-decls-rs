// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
impl Worker { fn new (id : usize , receiver : Arc < Mutex < mpsc :: Receiver < Job > > >) -> Worker { let thread = thread :: spawn (move | | { loop { let message = receiver . lock () . unwrap () . recv () ; match message { Ok (job) => { println ! ("Worker {id} got a job; executing.") ; job () ; } Err (_) => { println ! ("Worker {id} disconnected; shutting down.") ; break ; } } } }) ; Worker { id , thread } } }
};
}
