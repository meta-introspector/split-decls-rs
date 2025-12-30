// Generated macro for impl_530 (impl)
macro_rules! Depcrate_utilsimpl_530 {
() => {
// Module: crate::utils
// Provides: {"impl_530"}
// Dependencies: {}
# [allow (deprecated)] impl < T > Condition < T > where T : Clone , { pub fn wait (& mut self) -> oneshot :: Receiver < T > { let (tx , rx) = oneshot :: channel () ; self . waiters . push (tx) ; rx } pub fn set (self , result : T) { for waiter in self . waiters { let _ = waiter . send (result . clone ()) ; } } }
};
}
