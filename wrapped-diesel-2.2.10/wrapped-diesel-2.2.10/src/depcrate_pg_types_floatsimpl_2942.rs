// Generated macro for impl_2942 (impl)
macro_rules! Depcrate_pg_types_floatsimpl_2942 {
() => {
// Module: crate::pg::types::floats
// Provides: {"impl_2942"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl FromSql < sql_types :: Numeric , Pg > for PgNumeric { fn from_sql (bytes : PgValue < '_ >) -> deserialize :: Result < Self > { let mut bytes = bytes . as_bytes () ; let digit_count = bytes . read_u16 :: < NetworkEndian > () ? ; let mut digits = Vec :: with_capacity (digit_count as usize) ; let weight = bytes . read_i16 :: < NetworkEndian > () ? ; let sign = bytes . read_u16 :: < NetworkEndian > () ? ; let scale = bytes . read_u16 :: < NetworkEndian > () ? ; for _ in 0 .. digit_count { digits . push (bytes . read_i16 :: < NetworkEndian > () ?) ; } match sign { 0 => Ok (PgNumeric :: Positive { weight , scale , digits , }) , 0x4000 => Ok (PgNumeric :: Negative { weight , scale , digits , }) , 0xC000 => Ok (PgNumeric :: NaN) , invalid => Err (Box :: new (InvalidNumericSign (invalid))) , } } }
};
}
