// Generated macro for _64 (module)
macro_rules! Depcrate_64 {
() => {
// Module: crate
// Provides: {"_64"}
// Dependencies: {}
# [cfg (target_pointer_width = "64")] mod _64 { use crate :: { Error , From } ; promotion ! { i8 => f32 , f64 , i8 , i16 , i32 , i64 , isize ; i16 => f32 , f64 , i16 , i32 , i64 , isize ; i32 => f32 , f64 , i32 , i64 , isize ; i64 => f32 , f64 , i64 , isize ; isize => f32 , f64 , i64 , isize ; } half_promotion ! { i8 => u8 , u16 , u32 , u64 , usize ; i16 => u16 , u32 , u64 , usize ; i32 => u32 , u64 , usize ; i64 => u64 , usize ; isize => u64 , usize ; } from_signed ! { i16 => i8 , u8 ; i32 => i8 , i16 , u8 , u16 ; i64 => i8 , i16 , i32 , u8 , u16 , u32 ; isize => i8 , i16 , i32 , u8 , u16 , u32 ; } promotion ! { u8 => f32 , f64 , i16 , i32 , i64 , isize , u8 , u16 , u32 , u64 , usize ; u16 => f32 , f64 , i32 , i64 , isize , u16 , u32 , u64 , usize ; u32 => f32 , f64 , i64 , isize , u32 , u64 , usize ; u64 => f32 , f64 , u64 , usize ; usize => f32 , f64 , u64 , usize ; } from_unsigned ! { u8 => i8 ; u16 => i8 , i16 , u8 ; u32 => i8 , i16 , i32 , u8 , u16 ; u64 => i8 , i16 , i32 , i64 , isize , u8 , u16 , u32 ; usize => i8 , i16 , i32 , i64 , isize , u8 , u16 , u32 ; } promotion ! { f32 => f32 , f64 ; f64 => f64 ; } from_float ! { f32 => i8 , i16 , i32 , i64 , isize , u8 , u16 , u32 , u64 , usize ; f64 => i8 , i16 , i32 , i64 , isize , u8 , u16 , u32 , u64 , usize ; } }
};
}
