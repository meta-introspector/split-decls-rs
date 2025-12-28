macro_rules! deps {
    () => {
        BuiltinInt!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl fmt :: Display for BuiltinInt { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { BuiltinInt :: Isize => "isize" , BuiltinInt :: I8 => "i8" , BuiltinInt :: I16 => "i16" , BuiltinInt :: I32 => "i32" , BuiltinInt :: I64 => "i64" , BuiltinInt :: I128 => "i128" , }) } }
    };
}

impl_45!();