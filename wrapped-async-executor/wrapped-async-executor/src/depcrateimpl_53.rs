// Generated macro for impl_53 (impl)
macro_rules! Depcrateimpl_53 {
() => {
// Module: crate
// Provides: {"impl_53"}
// Dependencies: {}
impl State { # [doc = " Creates state for a new executor."] const fn new () -> State { State { queue : ConcurrentQueue :: unbounded () , local_queues : RwLock :: new (Vec :: new ()) , notified : AtomicBool :: new (true) , sleepers : Mutex :: new (Sleepers { count : 0 , wakers : Vec :: new () , free_ids : Vec :: new () , }) , active : Mutex :: new (Slab :: new ()) , } } fn pin (& self) -> Pin < & Self > { Pin :: new (self) } # [doc = " Returns a reference to currently active tasks."] fn active (self : Pin < & Self >) -> MutexGuard < '_ , Slab < Waker > > { self . get_ref () . active . lock () . unwrap_or_else (PoisonError :: into_inner) } # [doc = " Notifies a sleeping ticker."] # [inline] fn notify (& self) { if self . notified . compare_exchange (false , true , Ordering :: AcqRel , Ordering :: Acquire) . is_ok () { let waker = self . sleepers . lock () . unwrap_or_else (PoisonError :: into_inner) . notify () ; if let Some (w) = waker { w . wake () ; } } } pub (crate) fn try_tick (& self) -> bool { match self . queue . pop () { Err (_) => false , Ok (runnable) => { self . notify () ; runnable . run () ; true } } } pub (crate) async fn tick (& self) { let runnable = Ticker :: new (self) . runnable () . await ; runnable . run () ; } pub async fn run < T > (& self , future : impl Future < Output = T >) -> T { let mut runner = Runner :: new (self) ; let mut rng = fastrand :: Rng :: new () ; let run_forever = async { loop { for _ in 0 .. 200 { let runnable = runner . runnable (& mut rng) . await ; runnable . run () ; } future :: yield_now () . await ; } } ; future . or (run_forever) . await } }
};
}
