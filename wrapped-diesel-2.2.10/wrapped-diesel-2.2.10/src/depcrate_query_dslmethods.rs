// Generated macro for methods (module)
macro_rules! Depcrate_query_dslmethods {
() => {
// Module: crate::query_dsl
// Provides: {"methods"}
// Dependencies: {}
# [doc = " The traits used by `QueryDsl`."] # [doc = ""] # [doc = " Each trait in this module represents exactly one method from `QueryDsl`."] # [doc = " Apps should general rely on `QueryDsl` directly, rather than these traits."] # [doc = " However, generic code may need to include a where clause that references"] # [doc = " these traits."] pub mod methods { pub use super :: boxed_dsl :: BoxedDsl ; pub use super :: distinct_dsl :: * ; # [doc (inline)] pub use super :: filter_dsl :: * ; pub use super :: group_by_dsl :: GroupByDsl ; pub use super :: having_dsl :: HavingDsl ; pub use super :: limit_dsl :: LimitDsl ; pub use super :: load_dsl :: { ExecuteDsl , LoadQuery } ; pub use super :: locking_dsl :: { LockingDsl , ModifyLockDsl } ; pub use super :: nullable_select_dsl :: SelectNullableDsl ; pub use super :: offset_dsl :: OffsetDsl ; pub use super :: order_dsl :: { OrderDsl , ThenOrderDsl } ; pub use super :: select_dsl :: SelectDsl ; pub use super :: single_value_dsl :: SingleValueDsl ; # [cfg (all (feature = "with-deprecated" , not (feature = "without-deprecated")))] # [doc (hidden)] # [allow (deprecated)] # [deprecated (note = "Use `LoadQuery::RowIter` directly")] pub use super :: load_dsl :: LoadRet ; }
};
}
