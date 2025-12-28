macro_rules! deps {
    () => {
        BuiltinFloat!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl fmt :: Display for BuiltinFloat { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { BuiltinFloat :: F16 => "f16" , BuiltinFloat :: F32 => "f32" , BuiltinFloat :: F64 => "f64" , BuiltinFloat :: F128 => "f128" , }) } }
    };
}

impl_47!();