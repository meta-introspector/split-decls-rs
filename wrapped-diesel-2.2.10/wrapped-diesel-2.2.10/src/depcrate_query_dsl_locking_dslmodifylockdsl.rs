// Generated macro for ModifyLockDsl (trait)
macro_rules! Depcrate_query_dsl_locking_dslModifyLockDsl {
() => {
// Module: crate::query_dsl::locking_dsl
// Provides: {"ModifyLockDsl"}
// Dependencies: {}
# [doc = " Methods related to modifiers on locking select statements"] # [doc = ""] # [doc = " This trait should not be relied on directly by most apps. Its behavior is"] # [doc = " provided by [`QueryDsl`]. However, you may need a where clause on this trait"] # [doc = " to call `skip_locked` from generic code."] # [doc = ""] # [doc = " [`QueryDsl`]: crate::QueryDsl"] pub trait ModifyLockDsl < Modifier > { # [doc = " The type returned by `modify_lock`. See [`dsl::SkipLocked`] and friends"] # [doc = " for convenient access to this type."] # [doc = ""] # [doc = " [`dsl::SkipLocked`]: crate::dsl::SkipLocked"] type Output ; # [doc = " See the trait level documentation"] fn modify_lock (self , modifier : Modifier) -> Self :: Output ; }
};
}
