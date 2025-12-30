// Generated macro for impl_2943 (impl)
macro_rules! Depcrate_pg_types_floatsimpl_2943 {
() => {
// Module: crate::pg::types::floats
// Provides: {"impl_2943"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl ToSql < sql_types :: Numeric , Pg > for PgNumeric { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { let sign = match * self { PgNumeric :: Positive { .. } => 0 , PgNumeric :: Negative { .. } => 0x4000 , PgNumeric :: NaN => 0xC000 , } ; let empty_vec = Vec :: new () ; let digits = match * self { PgNumeric :: Positive { ref digits , .. } | PgNumeric :: Negative { ref digits , .. } => { digits } PgNumeric :: NaN => & empty_vec , } ; let weight = match * self { PgNumeric :: Positive { weight , .. } | PgNumeric :: Negative { weight , .. } => weight , PgNumeric :: NaN => 0 , } ; let scale = match * self { PgNumeric :: Positive { scale , .. } | PgNumeric :: Negative { scale , .. } => scale , PgNumeric :: NaN => 0 , } ; out . write_u16 :: < NetworkEndian > (digits . len () . try_into () ?) ? ; out . write_i16 :: < NetworkEndian > (weight) ? ; out . write_u16 :: < NetworkEndian > (sign) ? ; out . write_u16 :: < NetworkEndian > (scale) ? ; for digit in digits . iter () { out . write_i16 :: < NetworkEndian > (* digit) ? ; } Ok (IsNull :: No) } }
};
}
