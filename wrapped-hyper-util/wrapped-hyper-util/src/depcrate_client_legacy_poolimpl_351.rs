// Generated macro for impl_351 (impl)
macro_rules! Depcrate_client_legacy_poolimpl_351 {
() => {
// Module: crate::client::legacy::pool
// Provides: {"impl_351"}
// Dependencies: {}
impl < T , K : Key > Pool < T , K > { pub fn new < E , M > (config : Config , executor : E , timer : Option < M >) -> Pool < T , K > where E : hyper :: rt :: Executor < exec :: BoxSendFuture > + Send + Sync + Clone + 'static , M : hyper :: rt :: Timer + Send + Sync + Clone + 'static , { let exec = Exec :: new (executor) ; let timer = timer . map (| t | Timer :: new (t)) ; let inner = if config . is_enabled () { Some (Arc :: new (Mutex :: new (PoolInner { connecting : HashSet :: new () , idle : HashMap :: new () , idle_interval_ref : None , max_idle_per_host : config . max_idle_per_host , waiters : HashMap :: new () , exec , timer , timeout : config . idle_timeout , }))) } else { None } ; Pool { inner } } pub (crate) fn is_enabled (& self) -> bool { self . inner . is_some () } # [cfg (test)] pub (super) fn no_timer (& self) { { let mut inner = self . inner . as_ref () . unwrap () . lock () . unwrap () ; assert ! (inner . idle_interval_ref . is_none () , "timer already spawned") ; let (tx , _) = oneshot :: channel () ; inner . idle_interval_ref = Some (tx) ; } } }
};
}
