// Generated macro for impl_56 (impl)
macro_rules! Depcrate_deimpl_56 {
() => {
// Module: crate::de
// Provides: {"impl_56"}
// Dependencies: {}
impl < 'de > de :: Deserializer < 'de > for Config { type Error = ConfigError ; config_deserialize_via_value ! { deserialize_any ; deserialize_bool ; deserialize_i8 ; deserialize_i16 ; deserialize_i32 ; deserialize_i64 ; deserialize_u8 ; deserialize_u16 ; deserialize_u32 ; deserialize_u64 ; deserialize_f32 ; deserialize_f64 ; deserialize_str ; deserialize_string ; deserialize_option ; deserialize_char ; deserialize_seq ; deserialize_bytes ; deserialize_byte_buf ; deserialize_map ; deserialize_unit ; deserialize_identifier ; deserialize_ignored_any ; deserialize_enum (name : &'static str , variants : &'static [&'static str]) ; deserialize_unit_struct (name : &'static str) ; deserialize_newtype_struct (name : &'static str) ; deserialize_tuple (n : usize) ; deserialize_tuple_struct (name : &'static str , n : usize) ; deserialize_struct (name : &'static str , fields : &'static [&'static str]) ; } }
};
}
