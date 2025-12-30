// Generated macro for foldable_impls (macro)
macro_rules! Depcrate_sql_types_foldfoldable_impls {
() => {
// Module: crate::sql_types::fold
// Provides: {"foldable_impls"}
// Dependencies: {}
macro_rules ! foldable_impls { ($ ($ Source : ty => ($ SumType : ty , $ AvgType : ty)) ,+,) => { $ (impl Foldable for $ Source { type Sum = sql_types :: Nullable <$ SumType >; type Avg = sql_types :: Nullable <$ AvgType >; }) + } }
};
}
