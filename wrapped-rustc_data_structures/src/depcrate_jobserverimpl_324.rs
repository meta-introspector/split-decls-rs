// Generated macro for impl_324 (impl)
macro_rules! Depcrate_jobserverimpl_324 {
() => {
// Module: crate::jobserver
// Provides: {"impl_324"}
// Dependencies: {}
impl Proxy { pub fn new () -> Arc < Self > { let proxy = Arc :: new (Proxy { client : client () , data : Mutex :: new (ProxyData { used : 1 , pending : 0 }) , wake_pending : Condvar :: new () , helper : OnceLock :: new () , }) ; let proxy_ = Arc :: clone (& proxy) ; let helper = proxy . client . clone () . into_helper_thread (move | token | { if let Ok (token) = token { let mut data = proxy_ . data . lock () ; if data . pending > 0 { token . drop_without_releasing () ; assert ! (data . used > 0) ; data . used += 1 ; data . pending -= 1 ; proxy_ . wake_pending . notify_one () ; } else { drop (data) ; drop (token) ; } } }) . expect ("failed to create helper thread") ; proxy . helper . set (helper) . unwrap () ; proxy } pub fn acquire_thread (& self) { let mut data = self . data . lock () ; if data . used == 0 { assert_eq ! (data . pending , 0) ; data . used += 1 ; } else { self . helper . get () . unwrap () . request_token () ; data . pending += 1 ; self . wake_pending . wait (& mut data) ; } } pub fn release_thread (& self) { let mut data = self . data . lock () ; if data . pending > 0 { data . pending -= 1 ; self . wake_pending . notify_one () ; } else { data . used -= 1 ; if data . used > 0 { drop (data) ; self . client . release_raw () . ok () ; } } } }
};
}
