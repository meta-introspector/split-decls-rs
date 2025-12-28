macro_rules! float_ty_to_string {
    () => {
        pub fn float_ty_to_string (ty : FloatTy) -> & 'static str { match ty { FloatTy :: F16 => "f16" , FloatTy :: F32 => "f32" , FloatTy :: F64 => "f64" , FloatTy :: F128 => "f128" , } }
    };
}

float_ty_to_string!();