// Generated macro for option_to_sql (function)
macro_rules! Depcrate_type_impls_optionoption_to_sql {
() => {
// Module: crate::type_impls::option
// Provides: {"option_to_sql"}
// Dependencies: {}
# [test] # [cfg (feature = "postgres")] fn option_to_sql () { use crate :: pg :: Pg ; use crate :: query_builder :: bind_collector :: ByteWrapper ; use crate :: sql_types ; type Type = sql_types :: Nullable < sql_types :: VarChar > ; let mut buffer = Vec :: new () ; let is_null = { let mut bytes = Output :: test (ByteWrapper (& mut buffer)) ; ToSql :: < Type , Pg > :: to_sql (& None :: < String > , & mut bytes) . unwrap () } ; assert_eq ! (IsNull :: Yes , is_null) ; assert ! (buffer . is_empty ()) ; let is_null = { let mut bytes = Output :: test (ByteWrapper (& mut buffer)) ; ToSql :: < Type , Pg > :: to_sql (& Some ("") , & mut bytes) . unwrap () } ; assert_eq ! (IsNull :: No , is_null) ; assert ! (buffer . is_empty ()) ; let is_null = { let mut bytes = Output :: test (ByteWrapper (& mut buffer)) ; ToSql :: < Type , Pg > :: to_sql (& Some ("Sean") , & mut bytes) . unwrap () } ; let expected_bytes = b"Sean" . to_vec () ; assert_eq ! (IsNull :: No , is_null) ; assert_eq ! (buffer , expected_bytes) ; }
};
}
