macro_rules! deps {
    () => {
        Item!();
        String!();
        ToComponents!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        impl ToComponents for String { fn to_components (& self) -> impl Iterator < Item = & BStr > { self . split ('/') . map (Into :: into) } }
    };
}

impl_194!();