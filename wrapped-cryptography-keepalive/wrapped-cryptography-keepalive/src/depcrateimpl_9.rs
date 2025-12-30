// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
# [allow (clippy :: new_without_default)] impl < T : StableDeref > KeepAlive < T > { pub fn new () -> Self { KeepAlive { values : UnsafeCell :: new (vec ! []) , } } pub fn add (& self , v : T) -> & T :: Target { unsafe { let values = & mut * self . values . get () ; values . push (v) ; values . last () . unwrap () . deref () } } }
};
}
