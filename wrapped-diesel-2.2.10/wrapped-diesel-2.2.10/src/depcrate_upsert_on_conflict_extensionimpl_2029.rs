// Generated macro for impl_2029 (impl)
macro_rules! Depcrate_upsert_on_conflict_extensionimpl_2029 {
() => {
// Module: crate::upsert::on_conflict_extension
// Provides: {"impl_2029"}
// Dependencies: {}
impl < T : QuerySource , U , Op , Ret , Target > IncompleteOnConflict < InsertStatement < T , U , Op , Ret > , Target > { # [doc = " Creates a query with `ON CONFLICT (target) DO NOTHING`"] # [doc = ""] # [doc = " If you want to do nothing when *any* constraint conflicts, use"] # [doc = " [`on_conflict_do_nothing`] instead. See [`on_conflict`] for usage"] # [doc = " examples."] # [doc = ""] # [doc = " [`on_conflict_do_nothing`]: crate::query_builder::InsertStatement::on_conflict_do_nothing()"] # [doc = " [`on_conflict`]: crate::query_builder::InsertStatement::on_conflict()"] pub fn do_nothing (self ,) -> InsertStatement < T , OnConflictValues < U , Target , DoNothing < T > > , Op , Ret > { let target = self . target ; self . stmt . replace_values (| values | { OnConflictValues :: new (values , target , DoNothing :: new () , NoWhereClause) }) } }
};
}
