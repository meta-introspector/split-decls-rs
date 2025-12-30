// Generated macro for impl_contains_defaultable_value (macro)
macro_rules! Depcrate_query_builder_insert_statement_insert_with_default_for_sqliteimpl_contains_defaultable_value {
() => {
// Module: crate::query_builder::insert_statement::insert_with_default_for_sqlite
// Provides: {"impl_contains_defaultable_value"}
// Dependencies: {}
macro_rules ! impl_contains_defaultable_value { (@ build start_ts = [$ ($ ST : ident ,) *] , ts = [$ T1 : ident ,] , bounds = [$ ($ bounds : tt) *] , out = [$ ($ out : tt) *] ,) => { impl <$ ($ ST ,) *> ContainsDefaultableValue for ($ ($ ST ,) *) where $ ($ ST : ContainsDefaultableValue ,) * $ ($ bounds) * $ T1 :: Out : Any <$ ($ out) *>, { type Out = <$ T1 :: Out as Any <$ ($ out) *>>:: Out ; } } ; (@ build start_ts = [$ ($ ST : ident ,) *] , ts = [$ T1 : ident , $ ($ T : ident ,) +] , bounds = [$ ($ bounds : tt) *] , out = [$ ($ out : tt) *] ,) => { impl_contains_defaultable_value ! { @ build start_ts = [$ ($ ST ,) *] , ts = [$ ($ T ,) *] , bounds = [$ ($ bounds) * $ T1 :: Out : Any <$ ($ out) *>,] , out = [<$ T1 :: Out as Any <$ ($ out) *>>:: Out] , } } ; ($ T1 : ident , $ ($ T : ident ,) +) => { impl_contains_defaultable_value ! { @ build start_ts = [$ T1 , $ ($ T ,) *] , ts = [$ ($ T ,) *] , bounds = [] , out = [$ T1 :: Out] , } } ; ($ T1 : ident ,) => { impl <$ T1 > ContainsDefaultableValue for ($ T1 ,) where $ T1 : ContainsDefaultableValue , { type Out = <$ T1 as ContainsDefaultableValue >:: Out ; } } }
};
}
