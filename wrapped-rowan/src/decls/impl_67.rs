macro_rules! deps {
    () => {
        SyntaxNodeChildren!();
        Language!();
        SyntaxNode!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < L : Language > SyntaxNodeChildren < L > { pub fn by_kind (self , matcher : impl Fn (L :: Kind) -> bool) -> impl Iterator < Item = SyntaxNode < L > > { self . raw . by_kind (move | raw_kind | matcher (L :: kind_from_raw (raw_kind))) . map (SyntaxNode :: from) } }
    };
}

impl_67!();