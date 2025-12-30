// Generated macro for impl_2825 (impl)
macro_rules! Depcrate_pg_types_arrayimpl_2825 {
() => {
// Module: crate::pg::types::array
// Provides: {"impl_2825"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl < ST , T > ToSql < Array < ST > , Pg > for [T] where Pg : HasSqlType < ST > , T : ToSql < ST , Pg > , { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { let num_dimensions = 1 ; out . write_i32 :: < NetworkEndian > (num_dimensions) ? ; let flags = 0 ; out . write_i32 :: < NetworkEndian > (flags) ? ; let element_oid = Pg :: metadata (out . metadata_lookup ()) . oid () ? ; out . write_u32 :: < NetworkEndian > (element_oid) ? ; out . write_i32 :: < NetworkEndian > (self . len () . try_into () ?) ? ; let lower_bound = 1 ; out . write_i32 :: < NetworkEndian > (lower_bound) ? ; let mut buffer = Vec :: new () ; for elem in self . iter () { let is_null = { let mut temp_buffer = Output :: new (ByteWrapper (& mut buffer) , out . metadata_lookup ()) ; elem . to_sql (& mut temp_buffer) ? } ; if let IsNull :: No = is_null { out . write_i32 :: < NetworkEndian > (buffer . len () . try_into () ?) ? ; out . write_all (& buffer) ? ; buffer . clear () ; } else { out . write_i32 :: < NetworkEndian > (- 1) ? ; } } Ok (IsNull :: No) } }
};
}
