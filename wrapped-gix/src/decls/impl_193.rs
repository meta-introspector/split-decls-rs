macro_rules! deps {
    () => {
        ToComponents!();
        Item!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl ToComponents for & str { fn to_components (& self) -> impl Iterator < Item = & BStr > { self . split ('/') . map (Into :: into) } }
    };
}

impl_193!()