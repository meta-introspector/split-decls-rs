// Generated macro for impl_9 (impl)
macro_rules! Depcrate_writerimpl_9 {
() => {
// Module: crate::writer
// Provides: {"impl_9"}
// Dependencies: {}
impl < W : Write > TokenWrite for GenericWriter < W > { fn write_u8 (& mut self , value : u8) -> fmt :: Result { self . write_str (itoa :: Buffer :: new () . format (value)) } fn write_u16 (& mut self , value : u16) -> fmt :: Result { self . write_str (itoa :: Buffer :: new () . format (value)) } fn write_u32 (& mut self , value : u32) -> fmt :: Result { self . write_str (itoa :: Buffer :: new () . format (value)) } fn write_u64 (& mut self , value : u64) -> fmt :: Result { self . write_str (itoa :: Buffer :: new () . format (value)) } fn write_u128 (& mut self , value : u128) -> fmt :: Result { self . write_str (itoa :: Buffer :: new () . format (value)) } fn write_i8 (& mut self , value : i8) -> fmt :: Result { self . write_str (itoa :: Buffer :: new () . format (value)) } fn write_i16 (& mut self , value : i16) -> fmt :: Result { self . write_str (itoa :: Buffer :: new () . format (value)) } fn write_i32 (& mut self , value : i32) -> fmt :: Result { self . write_str (itoa :: Buffer :: new () . format (value)) } fn write_i64 (& mut self , value : i64) -> fmt :: Result { self . write_str (itoa :: Buffer :: new () . format (value)) } fn write_i128 (& mut self , value : i128) -> fmt :: Result { self . write_str (itoa :: Buffer :: new () . format (value)) } fn write_f32 (& mut self , value : f32) -> fmt :: Result { self . write_str (ryu :: Buffer :: new () . format (value)) } fn write_f64 (& mut self , value : f64) -> fmt :: Result { self . write_str (ryu :: Buffer :: new () . format (value)) } }
};
}
