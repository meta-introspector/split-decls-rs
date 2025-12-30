// Generated macro for impl_3107 (impl)
macro_rules! Depcrate_pg_types_rangesimpl_3107 {
() => {
// Module: crate::pg::types::ranges
// Provides: {"impl_3107"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl < ST , T > ToSql < Range < ST > , Pg > for (Bound < T > , Bound < T >) where T : ToSql < ST , Pg > , { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { let mut flags = match self . 0 { Bound :: Included (_) => RangeFlags :: LB_INC , Bound :: Excluded (_) => RangeFlags :: empty () , Bound :: Unbounded => RangeFlags :: LB_INF , } ; flags |= match self . 1 { Bound :: Included (_) => RangeFlags :: UB_INC , Bound :: Excluded (_) => RangeFlags :: empty () , Bound :: Unbounded => RangeFlags :: UB_INF , } ; out . write_u8 (flags . bits ()) ? ; let mut buffer = Vec :: new () ; match self . 0 { Bound :: Included (ref value) | Bound :: Excluded (ref value) => { { let mut inner_buffer = Output :: new (ByteWrapper (& mut buffer) , out . metadata_lookup ()) ; value . to_sql (& mut inner_buffer) ? ; } out . write_u32 :: < NetworkEndian > (buffer . len () . try_into () ?) ? ; out . write_all (& buffer) ? ; buffer . clear () ; } Bound :: Unbounded => { } } match self . 1 { Bound :: Included (ref value) | Bound :: Excluded (ref value) => { { let mut inner_buffer = Output :: new (ByteWrapper (& mut buffer) , out . metadata_lookup ()) ; value . to_sql (& mut inner_buffer) ? ; } out . write_u32 :: < NetworkEndian > (buffer . len () . try_into () ?) ? ; out . write_all (& buffer) ? ; } Bound :: Unbounded => { } } Ok (IsNull :: No) } }
};
}
