// Generated macro for bigdecimal (module)
macro_rules! Depcrate_type_impls_decimalbigdecimal {
() => {
// Module: crate::type_impls::decimal
// Provides: {"bigdecimal"}
// Dependencies: {}
# [cfg (feature = "numeric")] mod bigdecimal { extern crate bigdecimal ; use self :: bigdecimal :: BigDecimal ; use crate :: deserialize :: FromSqlRow ; use crate :: expression :: AsExpression ; use crate :: sql_types :: Numeric ; # [derive (AsExpression , FromSqlRow)] # [diesel (foreign_derive)] # [diesel (sql_type = Numeric)] struct BigDecimalProxy (BigDecimal) ; }
};
}
