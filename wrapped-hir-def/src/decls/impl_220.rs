macro_rules! deps {
    () => {
        FloatTypeWrapper!();
    };
}

macro_rules! impl_220 {
    () => {
        deps!();
        impl FloatTypeWrapper { pub fn new (sym : Symbol) -> Self { Self (sym) } pub fn to_f128 (& self) -> f128 { self . 0 . as_str () . parse () . unwrap_or_default () } pub fn to_f64 (& self) -> f64 { self . 0 . as_str () . parse () . unwrap_or_default () } pub fn to_f32 (& self) -> f32 { self . 0 . as_str () . parse () . unwrap_or_default () } pub fn to_f16 (& self) -> f16 { self . 0 . as_str () . parse () . unwrap_or_default () } }
    };
}

impl_220!()