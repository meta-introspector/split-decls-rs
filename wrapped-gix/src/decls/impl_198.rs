macro_rules! deps {
    () => {
        ToComponents!();
        Item!();
    };
}

macro_rules! impl_198 {
    () => {
        deps!();
        impl ToComponents for & BStr { fn to_components (& self) -> impl Iterator < Item = & BStr > { self . split (| b | * b == b'/') . map (Into :: into) } }
    };
}

impl_198!();