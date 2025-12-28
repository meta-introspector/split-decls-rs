macro_rules! int_ty_to_string {
    () => {
        pub fn int_ty_to_string (ty : IntTy) -> & 'static str { match ty { IntTy :: Isize => "isize" , IntTy :: I8 => "i8" , IntTy :: I16 => "i16" , IntTy :: I32 => "i32" , IntTy :: I64 => "i64" , IntTy :: I128 => "i128" , } }
    };
}

int_ty_to_string!()