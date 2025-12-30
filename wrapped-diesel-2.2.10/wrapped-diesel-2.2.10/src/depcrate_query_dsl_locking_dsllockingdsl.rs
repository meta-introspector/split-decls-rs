// Generated macro for LockingDsl (trait)
macro_rules! Depcrate_query_dsl_locking_dslLockingDsl {
() => {
// Module: crate::query_dsl::locking_dsl
// Provides: {"LockingDsl"}
// Dependencies: {}
# [doc = " Methods related to locking select statements"] # [doc = ""] # [doc = " This trait should not be relied on directly by most apps. Its behavior is"] # [doc = " provided by [`QueryDsl`]. However, you may need a where clause on this trait"] # [doc = " to call `for_update` from generic code."] # [doc = ""] # [doc = " [`QueryDsl`]: crate::QueryDsl"] pub trait LockingDsl < Lock > { # [doc = " The type returned by `set_lock`. See [`dsl::ForUpdate`] and friends for"] # [doc = " convenient access to this type."] # [doc = ""] # [doc = " [`dsl::ForUpdate`]: crate::dsl::ForUpdate"] type Output ; # [doc = " See the trait level documentation"] fn with_lock (self , lock : Lock) -> Self :: Output ; }
};
}
