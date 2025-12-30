// Generated macro for impl_60 (impl)
macro_rules! Depcrateimpl_60 {
() => {
// Module: crate
// Provides: {"impl_60"}
// Dependencies: {}
impl Runner < '_ > { # [doc = " Creates a runner and registers it in the executor state."] fn new (state : & State) -> Runner < '_ > { let runner = Runner { state , ticker : Ticker :: new (state) , local : Arc :: new (ConcurrentQueue :: bounded (512)) , ticks : 0 , } ; state . local_queues . write () . unwrap_or_else (PoisonError :: into_inner) . push (runner . local . clone ()) ; runner } # [doc = " Waits for the next runnable task to run."] async fn runnable (& mut self , rng : & mut fastrand :: Rng) -> Runnable { let runnable = self . ticker . runnable_with (| | { if let Ok (r) = self . local . pop () { return Some (r) ; } if let Ok (r) = self . state . queue . pop () { steal (& self . state . queue , & self . local) ; return Some (r) ; } if let Ok (local_queues) = self . state . local_queues . try_read () { let n = local_queues . len () ; let start = rng . usize (.. n) ; let iter = local_queues . iter () . chain (local_queues . iter ()) . skip (start) . take (n) ; let iter = iter . filter (| local | ! Arc :: ptr_eq (local , & self . local)) ; for local in iter { steal (local , & self . local) ; if let Ok (r) = self . local . pop () { return Some (r) ; } } } None }) . await ; self . ticks = self . ticks . wrapping_add (1) ; if self . ticks % 64 == 0 { steal (& self . state . queue , & self . local) ; } runnable } }
};
}
