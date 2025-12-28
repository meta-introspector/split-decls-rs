macro_rules! deps {
    () => {
        String!();
        Item!();
        ToComponents!();
    };
}

macro_rules! impl_195 {
    () => {
        deps!();
        impl ToComponents for & String { fn to_components (& self) -> impl Iterator < Item = & BStr > { self . split ('/') . map (Into :: into) } }
    };
}

impl_195!()