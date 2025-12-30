// Generated macro for impl_119 (impl)
macro_rules! Depcrateimpl_119 {
() => {
// Module: crate
// Provides: {"impl_119"}
// Dependencies: {}
impl < T : FromPrimitive > FromPrimitive for OrderedFloat < T > { fn from_i64 (n : i64) -> Option < Self > { T :: from_i64 (n) . map (OrderedFloat) } fn from_u64 (n : u64) -> Option < Self > { T :: from_u64 (n) . map (OrderedFloat) } fn from_isize (n : isize) -> Option < Self > { T :: from_isize (n) . map (OrderedFloat) } fn from_i8 (n : i8) -> Option < Self > { T :: from_i8 (n) . map (OrderedFloat) } fn from_i16 (n : i16) -> Option < Self > { T :: from_i16 (n) . map (OrderedFloat) } fn from_i32 (n : i32) -> Option < Self > { T :: from_i32 (n) . map (OrderedFloat) } fn from_usize (n : usize) -> Option < Self > { T :: from_usize (n) . map (OrderedFloat) } fn from_u8 (n : u8) -> Option < Self > { T :: from_u8 (n) . map (OrderedFloat) } fn from_u16 (n : u16) -> Option < Self > { T :: from_u16 (n) . map (OrderedFloat) } fn from_u32 (n : u32) -> Option < Self > { T :: from_u32 (n) . map (OrderedFloat) } fn from_f32 (n : f32) -> Option < Self > { T :: from_f32 (n) . map (OrderedFloat) } fn from_f64 (n : f64) -> Option < Self > { T :: from_f64 (n) . map (OrderedFloat) } }
};
}
