macro_rules! deps {
    () => {
        FloatType!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl FloatType { # [doc = " Returns the type corresponding to the given suffix (e.g. `\"f32\"` is"] # [doc = " mapped to `Self::F32`). If the suffix is not a valid float type, `None`"] # [doc = " is returned."] pub fn from_suffix (suffix : & str) -> Option < Self > { match suffix { "f32" => Some (FloatType :: F32) , "f64" => Some (FloatType :: F64) , _ => None , } } # [doc = " Returns the suffix for this type, e.g. `\"f32\"` for `Self::F32`."] pub fn suffix (self) -> & 'static str { match self { Self :: F32 => "f32" , Self :: F64 => "f64" , } } }
    };
}

impl_136!()