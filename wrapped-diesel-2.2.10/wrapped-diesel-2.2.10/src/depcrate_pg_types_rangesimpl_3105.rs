// Generated macro for impl_3105 (impl)
macro_rules! Depcrate_pg_types_rangesimpl_3105 {
() => {
// Module: crate::pg::types::ranges
// Provides: {"impl_3105"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl < T , ST > FromSql < Range < ST > , Pg > for (Bound < T > , Bound < T >) where T : FromSql < ST , Pg > , { fn from_sql (value : PgValue < '_ >) -> deserialize :: Result < Self > { let mut bytes = value . as_bytes () ; let flags : RangeFlags = RangeFlags :: from_bits_truncate (bytes . read_u8 () ?) ; let mut lower_bound = Bound :: Unbounded ; let mut upper_bound = Bound :: Unbounded ; if ! flags . contains (RangeFlags :: LB_INF) { let elem_size = bytes . read_i32 :: < NetworkEndian > () ? ; let (elem_bytes , new_bytes) = bytes . split_at (elem_size . try_into () ?) ; bytes = new_bytes ; let value = T :: from_sql (PgValue :: new_internal (elem_bytes , & value)) ? ; lower_bound = if flags . contains (RangeFlags :: LB_INC) { Bound :: Included (value) } else { Bound :: Excluded (value) } ; } if ! flags . contains (RangeFlags :: UB_INF) { let _size = bytes . read_i32 :: < NetworkEndian > () ? ; let value = T :: from_sql (PgValue :: new_internal (bytes , & value)) ? ; upper_bound = if flags . contains (RangeFlags :: UB_INC) { Bound :: Included (value) } else { Bound :: Excluded (value) } ; } Ok ((lower_bound , upper_bound)) } }
};
}
