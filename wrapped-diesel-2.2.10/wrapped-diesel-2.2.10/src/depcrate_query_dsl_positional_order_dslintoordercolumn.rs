// Generated macro for IntoOrderColumn (trait)
macro_rules! Depcrate_query_dsl_positional_order_dslIntoOrderColumn {
() => {
// Module: crate::query_dsl::positional_order_dsl
// Provides: {"IntoOrderColumn"}
// Dependencies: {}
pub trait IntoOrderColumn : Into < OrderColumn > { fn asc (self) -> Asc < OrderColumn > { Asc { expr : self . into () } } fn desc (self) -> Desc < OrderColumn > { Desc { expr : self . into () } } }
};
}
