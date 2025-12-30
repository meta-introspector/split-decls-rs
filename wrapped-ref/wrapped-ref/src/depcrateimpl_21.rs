// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl < V : sval :: Value > sval :: Value for Ref < V > { fn stream < 'sval , S : Stream < 'sval > + ? Sized > (& 'sval self , stream : & mut S) -> Result { self . 0 . stream (stream) } fn tag (& self) -> Option < sval :: Tag > { self . 0 . tag () } fn to_bool (& self) -> Option < bool > { self . 0 . to_bool () } fn to_f32 (& self) -> Option < f32 > { self . 0 . to_f32 () } fn to_f64 (& self) -> Option < f64 > { self . 0 . to_f64 () } fn to_i8 (& self) -> Option < i8 > { self . 0 . to_i8 () } fn to_i16 (& self) -> Option < i16 > { self . 0 . to_i16 () } fn to_i32 (& self) -> Option < i32 > { self . 0 . to_i32 () } fn to_i64 (& self) -> Option < i64 > { self . 0 . to_i64 () } fn to_i128 (& self) -> Option < i128 > { self . 0 . to_i128 () } fn to_u8 (& self) -> Option < u8 > { self . 0 . to_u8 () } fn to_u16 (& self) -> Option < u16 > { self . 0 . to_u16 () } fn to_u32 (& self) -> Option < u32 > { self . 0 . to_u32 () } fn to_u64 (& self) -> Option < u64 > { self . 0 . to_u64 () } fn to_u128 (& self) -> Option < u128 > { self . 0 . to_u128 () } fn to_text (& self) -> Option < & str > { self . 0 . to_text () } fn to_binary (& self) -> Option < & [u8] > { self . 0 . to_binary () } }
};
}
