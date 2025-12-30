// Generated macro for impl_2032 (impl)
macro_rules! Depcrate_upsert_on_conflict_extensionimpl_2032 {
() => {
// Module: crate::upsert::on_conflict_extension
// Provides: {"impl_2032"}
// Dependencies: {}
impl < T : QuerySource , U , Op , Ret , Target > IncompleteDoUpdate < InsertStatement < T , U , Op , Ret > , Target > { # [doc = " See [`do_update`] for usage examples."] # [doc = ""] # [doc = " [`do_update`]: IncompleteOnConflict::do_update()"] pub fn set < Changes > (self , changes : Changes ,) -> InsertStatement < T , OnConflictValues < U , Target , DoUpdate < Changes :: Changeset , T > > , Op , Ret > where T : QuerySource , Changes : AsChangeset < Target = T > , { let target = self . target ; self . stmt . replace_values (| values | { OnConflictValues :: new (values , target , DoUpdate :: new (changes . as_changeset ()) , NoWhereClause ,) }) } }
};
}
