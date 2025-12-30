// Generated macro for _32 (module)
macro_rules! Depcrate_32 {
() => {
// Module: crate
// Provides: {"_32"}
// Dependencies: {}
# [cfg (target_pointer_width = "32")] mod _32 { use crate :: { Error , From } ; promotion ! { i8 => f32 , f64 , i8 , i16 , i32 , isize , i64 ; i16 => f32 , f64 , i16 , i32 , isize , i64 ; i32 => f32 , f64 , i32 , isize , i64 ; isize => f32 , f64 , i32 , isize , i64 ; i64 => f32 , f64 , i64 ; } half_promotion ! { i8 => u8 , u16 , u32 , usize , u64 ; i16 => u16 , u32 , usize , u64 ; i32 => u32 , usize , u64 ; isize => u32 , usize , u64 ; i64 => u64 ; } from_signed ! { i16 => i8 , u8 ; i32 => i8 , i16 , u8 , u16 ; isize => i8 , i16 , u8 , u16 ; i64 => i8 , i16 , i32 , isize , u8 , u16 , u32 , usize ; } promotion ! { u8 => f32 , f64 , i16 , i32 , isize , i64 , u8 , u16 , u32 , usize , u64 ; u16 => f32 , f64 , i32 , isize , i64 , u16 , u32 , usize , u64 ; u32 => f32 , f64 , i64 , u32 , usize , u64 ; usize => f32 , f64 , i64 , u32 , usize , u64 ; u64 => f32 , f64 , u64 ; } from_unsigned ! { u8 => i8 ; u16 => i8 , i16 , u8 ; u32 => i8 , i16 , i32 , isize , u8 , u16 ; usize => i8 , i16 , i32 , isize , u8 , u16 ; u64 => i8 , i16 , i32 , isize , i64 , u8 , u16 , u32 , usize ; } promotion ! { f32 => f32 , f64 ; f64 => f64 ; } from_float ! { f32 => i8 , i16 , i32 , isize , i64 , u8 , u16 , u32 , usize , u64 ; f64 => i8 , i16 , i32 , isize , i64 , u8 , u16 , u32 , usize , u64 ; } }
};
}
