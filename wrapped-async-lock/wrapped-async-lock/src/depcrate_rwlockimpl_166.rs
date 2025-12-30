// Generated macro for impl_166 (impl)
macro_rules! Depcrate_rwlockimpl_166 {
() => {
// Module: crate::rwlock
// Provides: {"impl_166"}
// Dependencies: {}
impl < T : ? Sized > RwLockWriteGuardArc < T > { # [doc = " Consumes the lock (without dropping) and returns the underlying `Arc`."] # [inline] fn into_arc (guard : Self) -> Arc < RwLock < T > > { let guard = ManuallyDrop :: new (guard) ; unsafe { ptr :: read (& guard . lock) } } # [doc = " Downgrades into an upgradable reader guard."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " use std::sync::Arc;"] # [doc = " use async_lock::{RwLock, RwLockUpgradableReadGuardArc, RwLockWriteGuardArc};"] # [doc = ""] # [doc = " let lock = Arc::new(RwLock::new(1));"] # [doc = ""] # [doc = " let mut writer = lock.write_arc().await;"] # [doc = " *writer += 1;"] # [doc = ""] # [doc = " assert!(lock.try_read_arc().is_none());"] # [doc = ""] # [doc = " let reader = RwLockWriteGuardArc::downgrade_to_upgradable(writer);"] # [doc = " assert_eq!(*reader, 2);"] # [doc = ""] # [doc = " assert!(lock.try_write_arc().is_none());"] # [doc = " assert!(lock.try_read_arc().is_some());"] # [doc = ""] # [doc = " assert!(RwLockUpgradableReadGuardArc::try_upgrade(reader).is_ok())"] # [doc = " # })"] # [doc = " ```"] # [inline] pub fn downgrade_to_upgradable (guard : Self) -> RwLockUpgradableReadGuardArc < T > { unsafe { guard . lock . raw . downgrade_to_upgradable () ; } RwLockUpgradableReadGuardArc { lock : Self :: into_arc (guard) , } } }
};
}
