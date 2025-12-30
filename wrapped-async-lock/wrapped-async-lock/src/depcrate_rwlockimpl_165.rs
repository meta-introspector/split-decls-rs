// Generated macro for impl_165 (impl)
macro_rules! Depcrate_rwlockimpl_165 {
() => {
// Module: crate::rwlock
// Provides: {"impl_165"}
// Dependencies: {}
impl < T > RwLockWriteGuardArc < T > { # [doc = " Downgrades into a regular reader guard."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " use std::sync::Arc;"] # [doc = " use async_lock::{RwLock, RwLockWriteGuardArc};"] # [doc = ""] # [doc = " let lock = Arc::new(RwLock::new(1));"] # [doc = ""] # [doc = " let mut writer = lock.write_arc().await;"] # [doc = " *writer += 1;"] # [doc = ""] # [doc = " assert!(lock.try_read_arc().is_none());"] # [doc = ""] # [doc = " let reader = RwLockWriteGuardArc::downgrade(writer);"] # [doc = " assert_eq!(*reader, 2);"] # [doc = ""] # [doc = " assert!(lock.try_read_arc().is_some());"] # [doc = " # })"] # [doc = " ```"] # [inline] pub fn downgrade (guard : Self) -> RwLockReadGuardArc < T > { unsafe { guard . lock . raw . downgrade_write () ; } unsafe { RwLockReadGuardArc :: from_arc (Self :: into_arc (guard)) } } }
};
}
