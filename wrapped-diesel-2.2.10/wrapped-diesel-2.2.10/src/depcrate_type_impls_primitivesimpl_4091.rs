// Generated macro for impl_4091 (impl)
macro_rules! Depcrate_type_impls_primitivesimpl_4091 {
() => {
// Module: crate::type_impls::primitives
// Provides: {"impl_4091"}
// Dependencies: {}
impl < DB > ToSql < sql_types :: Text , DB > for str where for < 'a > DB : Backend < BindCollector < 'a > = RawBytesBindCollector < DB > > , { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , DB >) -> serialize :: Result { out . write_all (self . as_bytes ()) . map (| _ | IsNull :: No) . map_err (| e | Box :: new (e) as Box < dyn Error + Send + Sync >) } }
};
}
