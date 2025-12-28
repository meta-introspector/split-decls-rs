macro_rules! deps {
    () => {
        TokenStream!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        impl FromIterator < Self > for TokenStream { fn from_iter < I : IntoIterator < Item = Self > > (iter : I) -> Self { iter . into_iter () . fold (None , | accum : Option < Self > , n | { let mut ts = accum . unwrap_or_default () ; ts . combine (& n) ; Some (ts) }) . unwrap_or_else (Self :: new) } }
    };
}

impl_138!()