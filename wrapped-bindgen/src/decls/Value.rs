macro_rules! deps {
    () => {
        TypeName!();
    };
}

macro_rules! Value {
    () => {
        deps!();
        # [derive (Debug)] pub enum Value { Bool (bool) , U8 (u8) , I8 (i8) , U16 (u16) , I16 (i16) , U32 (u32) , I32 (i32) , U64 (u64) , I64 (i64) , F32 (f32) , F64 (f64) , Str (& 'static str) , String (String) , TypeName (TypeName) , }
    };
}

Value!()