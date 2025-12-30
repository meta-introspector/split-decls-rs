// Generated macro for impl_150 (impl)
macro_rules! Depcrate_rwlockimpl_150 {
() => {
// Module: crate::rwlock
// Provides: {"impl_150"}
// Dependencies: {}
impl < T > RwLockUpgradableReadGuardArc < T > { # [doc = " Downgrades into a regular reader guard."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " use std::sync::Arc;"] # [doc = " use async_lock::{RwLock, RwLockUpgradableReadGuardArc};"] # [doc = ""] # [doc = " let lock = Arc::new(RwLock::new(1));"] # [doc = ""] # [doc = " let reader = lock.upgradable_read_arc().await;"] # [doc = " assert_eq!(*reader, 1);"] # [doc = ""] # [doc = " assert!(lock.try_upgradable_read_arc().is_none());"] # [doc = ""] # [doc = " let reader = RwLockUpgradableReadGuardArc::downgrade(reader);"] # [doc = ""] # [doc = " assert!(lock.try_upgradable_read_arc().is_some());"] # [doc = " # })"] # [doc = " ```"] # [inline] pub fn downgrade (guard : Self) -> RwLockReadGuardArc < T > { unsafe { guard . lock . raw . downgrade_upgradable_read () ; } unsafe { RwLockReadGuardArc :: from_arc (Self :: into_arc (guard)) } } }
};
}
