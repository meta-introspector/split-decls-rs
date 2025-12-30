// Generated macro for impl_196 (impl)
macro_rules! Depcrateimpl_196 {
() => {
// Module: crate
// Provides: {"impl_196"}
// Dependencies: {}
impl < T : FloatCore + FromPrimitive > FromPrimitive for NotNan < T > { fn from_i64 (n : i64) -> Option < Self > { T :: from_i64 (n) . and_then (| n | NotNan :: new (n) . ok ()) } fn from_u64 (n : u64) -> Option < Self > { T :: from_u64 (n) . and_then (| n | NotNan :: new (n) . ok ()) } fn from_isize (n : isize) -> Option < Self > { T :: from_isize (n) . and_then (| n | NotNan :: new (n) . ok ()) } fn from_i8 (n : i8) -> Option < Self > { T :: from_i8 (n) . and_then (| n | NotNan :: new (n) . ok ()) } fn from_i16 (n : i16) -> Option < Self > { T :: from_i16 (n) . and_then (| n | NotNan :: new (n) . ok ()) } fn from_i32 (n : i32) -> Option < Self > { T :: from_i32 (n) . and_then (| n | NotNan :: new (n) . ok ()) } fn from_usize (n : usize) -> Option < Self > { T :: from_usize (n) . and_then (| n | NotNan :: new (n) . ok ()) } fn from_u8 (n : u8) -> Option < Self > { T :: from_u8 (n) . and_then (| n | NotNan :: new (n) . ok ()) } fn from_u16 (n : u16) -> Option < Self > { T :: from_u16 (n) . and_then (| n | NotNan :: new (n) . ok ()) } fn from_u32 (n : u32) -> Option < Self > { T :: from_u32 (n) . and_then (| n | NotNan :: new (n) . ok ()) } fn from_f32 (n : f32) -> Option < Self > { T :: from_f32 (n) . and_then (| n | NotNan :: new (n) . ok ()) } fn from_f64 (n : f64) -> Option < Self > { T :: from_f64 (n) . and_then (| n | NotNan :: new (n) . ok ()) } }
};
}
