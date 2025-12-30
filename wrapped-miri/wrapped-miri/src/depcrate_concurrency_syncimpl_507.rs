// Generated macro for impl_507 (impl)
macro_rules! Depcrate_concurrency_syncimpl_507 {
() => {
// Module: crate::concurrency::sync
// Provides: {"impl_507"}
// Dependencies: {}
impl RwLock { # [inline] # [doc = " Check if locked."] fn is_locked (& self) -> bool { trace ! ("rwlock_is_locked: writer is {:?} and there are {} reader threads (some of which could hold multiple read locks)" , self . writer , self . readers . len () ,) ; self . writer . is_some () || self . readers . is_empty () . not () } # [doc = " Check if write locked."] # [inline] fn is_write_locked (& self) -> bool { trace ! ("rwlock_is_write_locked: writer is {:?}" , self . writer) ; self . writer . is_some () } }
};
}
