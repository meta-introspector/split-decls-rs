// Generated macro for test (module)
macro_rules! Depcrate_arbitrary__alloc_synctest {
() => {
// Module: crate::arbitrary::_alloc::sync
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { no_panic_test ! (arc => Arc < u8 >, atomic_bool => AtomicBool , atomic_isize => AtomicIsize , atomic_usize => AtomicUsize , ordering => Ordering) ; # [cfg (feature = "unstable")] no_panic_test ! (atomic_i8 => AtomicI8 , atomic_i16 => AtomicI16 , atomic_i32 => AtomicI32 , atomic_u8 => AtomicU8 , atomic_u16 => AtomicU16 , atomic_u32 => AtomicU32) ; # [cfg (all (feature = "unstable" , feature = "atomic64bit"))] no_panic_test ! (atomic_i64 => AtomicI64 , atomic_u64 => AtomicU64) ; }
};
}
