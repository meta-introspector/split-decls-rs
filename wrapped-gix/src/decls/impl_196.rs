macro_rules! deps {
    () => {
        Item!();
        ToComponents!();
    };
}

macro_rules! impl_196 {
    () => {
        deps!();
        impl ToComponents for BString { fn to_components (& self) -> impl Iterator < Item = & BStr > { self . split (| b | * b == b'/') . map (Into :: into) } }
    };
}

impl_196!();