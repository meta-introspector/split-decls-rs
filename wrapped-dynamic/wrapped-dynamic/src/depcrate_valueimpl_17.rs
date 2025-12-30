// Generated macro for impl_17 (impl)
macro_rules! Depcrate_valueimpl_17 {
() => {
// Module: crate::value
// Provides: {"impl_17"}
// Dependencies: {}
impl < T : sval :: Value > private :: DispatchValue for T { fn dispatch_stream < 'sval > (& 'sval self , stream : & mut dyn Stream < 'sval >) -> sval :: Result { self . stream (stream) } fn dispatch_tag (& self) -> Option < sval :: Tag > { self . tag () } fn dispatch_to_bool (& self) -> Option < bool > { self . to_bool () } fn dispatch_to_f32 (& self) -> Option < f32 > { self . to_f32 () } fn dispatch_to_f64 (& self) -> Option < f64 > { self . to_f64 () } fn dispatch_to_i8 (& self) -> Option < i8 > { self . to_i8 () } fn dispatch_to_i16 (& self) -> Option < i16 > { self . to_i16 () } fn dispatch_to_i32 (& self) -> Option < i32 > { self . to_i32 () } fn dispatch_to_i64 (& self) -> Option < i64 > { self . to_i64 () } fn dispatch_to_i128 (& self) -> Option < i128 > { self . to_i128 () } fn dispatch_to_u8 (& self) -> Option < u8 > { self . to_u8 () } fn dispatch_to_u16 (& self) -> Option < u16 > { self . to_u16 () } fn dispatch_to_u32 (& self) -> Option < u32 > { self . to_u32 () } fn dispatch_to_u64 (& self) -> Option < u64 > { self . to_u64 () } fn dispatch_to_u128 (& self) -> Option < u128 > { self . to_u128 () } fn dispatch_to_text (& self) -> Option < & str > { self . to_text () } fn dispatch_to_binary (& self) -> Option < & [u8] > { self . to_binary () } }
};
}
