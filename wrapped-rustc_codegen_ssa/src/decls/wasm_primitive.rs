macro_rules! wasm_primitive {
    () => {
        fn wasm_primitive (primitive : Primitive , ptr_type : & 'static str) -> & 'static str { match primitive { Primitive :: Int (integer , _) => match integer { Integer :: I8 | Integer :: I16 | Integer :: I32 => "i32" , Integer :: I64 => "i64" , Integer :: I128 => "i64, i64" , } , Primitive :: Float (float) => match float { Float :: F16 | Float :: F32 => "f32" , Float :: F64 => "f64" , Float :: F128 => "i64, i64" , } , Primitive :: Pointer (_) => ptr_type , } }
    };
}

wasm_primitive!();