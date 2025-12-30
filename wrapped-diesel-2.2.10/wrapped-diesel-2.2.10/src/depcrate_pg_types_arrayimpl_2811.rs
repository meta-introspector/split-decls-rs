// Generated macro for impl_2811 (impl)
macro_rules! Depcrate_pg_types_arrayimpl_2811 {
() => {
// Module: crate::pg::types::array
// Provides: {"impl_2811"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl < T , ST > FromSql < Array < ST > , Pg > for Vec < T > where T : FromSql < ST , Pg > , { fn from_sql (value : PgValue < '_ >) -> deserialize :: Result < Self > { let mut bytes = value . as_bytes () ; let num_dimensions = bytes . read_i32 :: < NetworkEndian > () ? ; let has_null = bytes . read_i32 :: < NetworkEndian > () ? != 0 ; let _oid = bytes . read_i32 :: < NetworkEndian > () ? ; if num_dimensions == 0 { return Ok (Vec :: new ()) ; } let num_elements = bytes . read_i32 :: < NetworkEndian > () ? ; let _lower_bound = bytes . read_i32 :: < NetworkEndian > () ? ; if num_dimensions != 1 { return Err ("multi-dimensional arrays are not supported" . into ()) ; } (0 .. num_elements) . map (| _ | { let elem_size = bytes . read_i32 :: < NetworkEndian > () ? ; if has_null && elem_size == - 1 { T :: from_nullable_sql (None) } else { let (elem_bytes , new_bytes) = bytes . split_at (elem_size . try_into () ?) ; bytes = new_bytes ; T :: from_sql (PgValue :: new_internal (elem_bytes , & value)) } }) . collect () } }
};
}
