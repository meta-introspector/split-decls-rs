// Generated macro for impl_182 (impl)
macro_rules! Depcrate_num_traitsimpl_182 {
() => {
// Module: crate::num_traits
// Provides: {"impl_182"}
// Dependencies: {}
impl ToPrimitive for bf16 { # [inline] fn to_i64 (& self) -> Option < i64 > { Self :: to_f32 (* self) . to_i64 () } # [inline] fn to_u64 (& self) -> Option < u64 > { Self :: to_f32 (* self) . to_u64 () } # [inline] fn to_i8 (& self) -> Option < i8 > { Self :: to_f32 (* self) . to_i8 () } # [inline] fn to_u8 (& self) -> Option < u8 > { Self :: to_f32 (* self) . to_u8 () } # [inline] fn to_i16 (& self) -> Option < i16 > { Self :: to_f32 (* self) . to_i16 () } # [inline] fn to_u16 (& self) -> Option < u16 > { Self :: to_f32 (* self) . to_u16 () } # [inline] fn to_i32 (& self) -> Option < i32 > { Self :: to_f32 (* self) . to_i32 () } # [inline] fn to_u32 (& self) -> Option < u32 > { Self :: to_f32 (* self) . to_u32 () } # [inline] fn to_f32 (& self) -> Option < f32 > { Some (Self :: to_f32 (* self)) } # [inline] fn to_f64 (& self) -> Option < f64 > { Some (Self :: to_f64 (* self)) } }
};
}
