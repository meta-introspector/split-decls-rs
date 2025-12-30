// Generated macro for _x128 (module)
macro_rules! Depcrate_x128 {
() => {
// Module: crate
// Provides: {"_x128"}
// Dependencies: {}
mod _x128 { use crate :: { Error , From } ; promotion ! { i8 => i128 ; i16 => i128 ; i32 => i128 ; i64 => i128 ; isize => i128 ; i128 => f32 , f64 , i128 ; } half_promotion ! { i8 => u128 ; i16 => u128 ; i32 => u128 ; i64 => u128 ; isize => u128 ; i128 => u128 ; } from_signed ! { i128 => i8 , i16 , i32 , i64 , isize , u8 , u16 , u32 , u64 , usize ; } promotion ! { u8 => i128 , u128 ; u16 => i128 , u128 ; u32 => i128 , u128 ; u64 => i128 , u128 ; usize => i128 , u128 ; u128 => f64 , u128 ; } from_unsigned ! { u128 => f32 , i8 , i16 , i32 , i64 , i128 , isize , u8 , u16 , u32 , u64 , usize ; } from_float_dst ! { f32 => u128 ; } from_float ! { f32 => i128 ; f64 => i128 , u128 ; } }
};
}
