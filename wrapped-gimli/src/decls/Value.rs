macro_rules! Value {
    () => {
        # [doc = " The value of an entry on the DWARF stack."] # [derive (Debug , Clone , Copy , PartialEq)] pub enum Value { # [doc = " A generic value, which is address-sized and of unspecified sign."] Generic (u64) , # [doc = " A signed 8-bit integer value."] I8 (i8) , # [doc = " An unsigned 8-bit integer value."] U8 (u8) , # [doc = " A signed 16-bit integer value."] I16 (i16) , # [doc = " An unsigned 16-bit integer value."] U16 (u16) , # [doc = " A signed 32-bit integer value."] I32 (i32) , # [doc = " An unsigned 32-bit integer value."] U32 (u32) , # [doc = " A signed 64-bit integer value."] I64 (i64) , # [doc = " An unsigned 64-bit integer value."] U64 (u64) , # [doc = " A 32-bit floating point value."] F32 (f32) , # [doc = " A 64-bit floating point value."] F64 (f64) , }
    };
}

Value!();