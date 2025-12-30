// Generated macro for impl_4096 (impl)
macro_rules! Depcrate_type_impls_primitivesimpl_4096 {
() => {
// Module: crate::type_impls::primitives
// Provides: {"impl_4096"}
// Dependencies: {}
impl < DB > ToSql < sql_types :: Binary , DB > for [u8] where for < 'a > DB : Backend < BindCollector < 'a > = RawBytesBindCollector < DB > > , { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , DB >) -> serialize :: Result { out . write_all (self) . map (| _ | IsNull :: No) . map_err (| e | Box :: new (e) as Box < dyn Error + Send + Sync >) } }
};
}
