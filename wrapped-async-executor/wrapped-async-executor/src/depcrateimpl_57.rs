// Generated macro for impl_57 (impl)
macro_rules! Depcrateimpl_57 {
() => {
// Module: crate
// Provides: {"impl_57"}
// Dependencies: {}
impl Ticker < '_ > { # [doc = " Creates a ticker."] fn new (state : & State) -> Ticker < '_ > { Ticker { state , sleeping : 0 } } # [doc = " Moves the ticker into sleeping and unnotified state."] # [doc = ""] # [doc = " Returns `false` if the ticker was already sleeping and unnotified."] fn sleep (& mut self , waker : & Waker) -> bool { let mut sleepers = self . state . sleepers . lock () . unwrap_or_else (PoisonError :: into_inner) ; match self . sleeping { 0 => { self . sleeping = sleepers . insert (waker) ; } id => { if ! sleepers . update (id , waker) { return false ; } } } self . state . notified . store (sleepers . is_notified () , Ordering :: Release) ; true } # [doc = " Moves the ticker into woken state."] fn wake (& mut self) { if self . sleeping != 0 { let mut sleepers = self . state . sleepers . lock () . unwrap_or_else (PoisonError :: into_inner) ; sleepers . remove (self . sleeping) ; self . state . notified . store (sleepers . is_notified () , Ordering :: Release) ; } self . sleeping = 0 ; } # [doc = " Waits for the next runnable task to run."] async fn runnable (& mut self) -> Runnable { self . runnable_with (| | self . state . queue . pop () . ok ()) . await } # [doc = " Waits for the next runnable task to run, given a function that searches for a task."] async fn runnable_with (& mut self , mut search : impl FnMut () -> Option < Runnable >) -> Runnable { future :: poll_fn (| cx | { loop { match search () { None => { if ! self . sleep (cx . waker ()) { return Poll :: Pending ; } } Some (r) => { self . wake () ; self . state . notify () ; return Poll :: Ready (r) ; } } } }) . await } }
};
}
