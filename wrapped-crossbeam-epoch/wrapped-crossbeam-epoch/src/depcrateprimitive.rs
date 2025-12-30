// Generated macro for primitive (module)
macro_rules! Depcrateprimitive {
() => {
// Module: crate
// Provides: {"primitive"}
// Dependencies: {}
# [cfg (target_has_atomic = "ptr")] # [cfg (not (crossbeam_loom))] # [allow (unused_imports , dead_code)] mod primitive { pub (crate) mod cell { # [derive (Debug)] # [repr (transparent)] pub (crate) struct UnsafeCell < T > (:: core :: cell :: UnsafeCell < T >) ; impl < T > UnsafeCell < T > { # [inline] pub (crate) const fn new (data : T) -> Self { Self (:: core :: cell :: UnsafeCell :: new (data)) } # [inline] pub (crate) fn with < R > (& self , f : impl FnOnce (* const T) -> R) -> R { f (self . 0 . get ()) } # [inline] pub (crate) fn with_mut < R > (& self , f : impl FnOnce (* mut T) -> R) -> R { f (self . 0 . get ()) } } } pub (crate) mod sync { # [cfg (feature = "alloc")] pub (crate) use alloc :: sync :: Arc ; pub (crate) use core :: sync :: atomic ; } # [cfg (feature = "std")] pub (crate) use std :: thread_local ; }
};
}
