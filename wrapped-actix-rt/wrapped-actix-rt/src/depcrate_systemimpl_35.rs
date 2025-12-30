// Generated macro for impl_35 (impl)
macro_rules! Depcrate_systemimpl_35 {
() => {
// Module: crate::system
// Provides: {"impl_35"}
// Dependencies: {}
# [cfg (not (feature = "io-uring"))] impl System { # [doc = " Create a new system."] # [doc = ""] # [doc = " # Panics"] # [doc = " Panics if underlying Tokio runtime can not be created."] # [allow (clippy :: new_ret_no_self)] pub fn new () -> SystemRunner { Self :: with_tokio_rt (| | { crate :: runtime :: default_tokio_runtime () . expect ("Default Actix (Tokio) runtime could not be created.") }) } # [doc = " Create a new System using the [Tokio Runtime](tokio-runtime) returned from a closure."] # [doc = ""] # [doc = " [tokio-runtime]: tokio::runtime::Runtime"] pub fn with_tokio_rt < F > (runtime_factory : F) -> SystemRunner where F : FnOnce () -> tokio :: runtime :: Runtime , { let (stop_tx , stop_rx) = oneshot :: channel () ; let (sys_tx , sys_rx) = mpsc :: unbounded_channel () ; let rt = crate :: runtime :: Runtime :: from (runtime_factory ()) ; let sys_arbiter = rt . block_on (async { Arbiter :: in_new_system () }) ; let system = System :: construct (sys_tx , sys_arbiter . clone ()) ; system . tx () . send (SystemCommand :: RegisterArbiter (usize :: MAX , sys_arbiter)) . unwrap () ; let sys_ctrl = SystemController :: new (sys_rx , stop_tx) ; rt . spawn (sys_ctrl) ; SystemRunner { rt , stop_rx } } }
};
}
