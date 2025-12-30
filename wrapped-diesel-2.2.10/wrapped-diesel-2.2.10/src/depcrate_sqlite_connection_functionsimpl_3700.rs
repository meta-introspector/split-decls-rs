// Generated macro for impl_3700 (impl)
macro_rules! Depcrate_sqlite_connection_functionsimpl_3700 {
() => {
// Module: crate::sqlite::connection::functions
// Provides: {"impl_3700"}
// Dependencies: {}
impl < 'a > Field < 'a , Sqlite > for FunctionArgument < 'a > { fn field_name (& self) -> Option < & str > { None } fn is_null (& self) -> bool { self . value () . is_none () } fn value (& self) -> Option < < Sqlite as Backend > :: RawValue < '_ > > { SqliteValue :: new (Ref :: map (Ref :: clone (& self . args) , | drop | std :: ops :: Deref :: deref (drop)) , self . col_idx ,) } }
};
}
