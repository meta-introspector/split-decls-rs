// Generated macro for impl_500 (impl)
macro_rules! Depcrate_ffi_taskimpl_500 {
() => {
// Module: crate::ffi::task
// Provides: {"impl_500"}
// Dependencies: {}
impl hyper_executor { fn new () -> Arc < hyper_executor > { Arc :: new (hyper_executor { driver : Mutex :: new (FuturesUnordered :: new ()) , spawn_queue : Mutex :: new (Vec :: new ()) , is_woken : Arc :: new (ExecWaker (AtomicBool :: new (false))) , }) } pub (crate) fn downgrade (exec : & Arc < hyper_executor >) -> WeakExec { WeakExec (Arc :: downgrade (exec)) } fn spawn (& self , task : Box < hyper_task >) { self . spawn_queue . lock () . unwrap () . push (TaskFuture { task : Some (task) }) ; } fn poll_next (& self) -> Option < Box < hyper_task > > { self . drain_queue () ; let waker = futures_util :: task :: waker_ref (& self . is_woken) ; let mut cx = Context :: from_waker (& waker) ; loop { { let mut driver = self . driver . lock () . unwrap () ; match Pin :: new (& mut * driver) . poll_next (& mut cx) { Poll :: Ready (val) => return val , Poll :: Pending => { } } ; } if self . drain_queue () { continue ; } if self . is_woken . 0 . swap (false , Ordering :: SeqCst) { continue ; } return None ; } } # [doc = " drain_queue locks both self.spawn_queue and self.driver, so it requires"] # [doc = " that neither of them be locked already."] fn drain_queue (& self) -> bool { let mut queue = self . spawn_queue . lock () . unwrap () ; if queue . is_empty () { return false ; } let driver = self . driver . lock () . unwrap () ; for task in queue . drain (..) { driver . push (task) ; } true } }
};
}
