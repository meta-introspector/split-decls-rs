macro_rules! deps {
    () => {
        SyntaxElementChildren!();
        SyntaxElement!();
        Language!();
        NodeOrToken!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl < L : Language > SyntaxElementChildren < L > { pub fn by_kind (self , matcher : impl Fn (L :: Kind) -> bool ,) -> impl Iterator < Item = SyntaxElement < L > > { self . raw . by_kind (move | raw_kind | matcher (L :: kind_from_raw (raw_kind))) . map (NodeOrToken :: from) } }
    };
}

impl_70!();