macro_rules! deps {
    () => {
        BuiltinFloat!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        # [rustfmt :: skip] impl BuiltinFloat { pub fn from_suffix (suffix : & str) -> Option < BuiltinFloat > { let res = match suffix { "f16" => BuiltinFloat :: F16 , "f32" => BuiltinFloat :: F32 , "f64" => BuiltinFloat :: F64 , "f128" => BuiltinFloat :: F128 , _ => return None , } ; Some (res) } }
    };
}

impl_44!()