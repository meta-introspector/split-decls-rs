macro_rules! Value {
    () => {
        # [derive (Debug , PartialEq)] pub enum Value { Bool (bool) , U8 (u8) , I8 (i8) , U16 (u16) , I16 (i16) , U32 (u32) , I32 (i32) , U64 (u64) , I64 (i64) , F32 (f32) , F64 (f64) , Utf8 (String) , Utf16 (String) , AttributeEnum (String , i32) , }
    };
}

Value!()