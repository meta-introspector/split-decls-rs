// Generated macro for impl_sql_type (macro)
macro_rules! Depcrate_type_impls_tuplesimpl_sql_type {
() => {
// Module: crate::type_impls::tuples
// Provides: {"impl_sql_type"}
// Dependencies: {}
macro_rules ! impl_sql_type { (@ build start_ts = [$ ($ ST : ident ,) *] , ts = [$ T1 : ident ,] , bounds = [$ ($ bounds : tt) *] , is_null = [$ ($ is_null : tt) *] ,) => { impl <$ ($ ST ,) *> SqlType for ($ ($ ST ,) *) where $ ($ ST : SqlType ,) * $ ($ bounds) * $ T1 :: IsNull : OneIsNullable <$ ($ is_null) *>, { type IsNull = <$ T1 :: IsNull as OneIsNullable <$ ($ is_null) *>>:: Out ; } } ; (@ build start_ts = [$ ($ ST : ident ,) *] , ts = [$ T1 : ident , $ ($ T : ident ,) +] , bounds = [$ ($ bounds : tt) *] , is_null = [$ ($ is_null : tt) *] ,) => { impl_sql_type ! { @ build start_ts = [$ ($ ST ,) *] , ts = [$ ($ T ,) *] , bounds = [$ ($ bounds) * $ T1 :: IsNull : OneIsNullable <$ ($ is_null) *>,] , is_null = [<$ T1 :: IsNull as OneIsNullable <$ ($ is_null) *>>:: Out] , } } ; ($ T1 : ident , $ ($ T : ident ,) +) => { impl_sql_type ! { @ build start_ts = [$ T1 , $ ($ T ,) *] , ts = [$ ($ T ,) *] , bounds = [] , is_null = [$ T1 :: IsNull] , } } ; ($ T1 : ident ,) => { impl <$ T1 > SqlType for ($ T1 ,) where $ T1 : SqlType , { type IsNull = $ T1 :: IsNull ; } } }
};
}
