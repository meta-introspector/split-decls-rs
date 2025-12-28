macro_rules! deps {
    () => {
        Language!();
        SyntaxNode!();
        Preorder!();
        WalkEvent!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl < L : Language > Iterator for Preorder < L > { type Item = WalkEvent < SyntaxNode < L > > ; fn next (& mut self) -> Option < Self :: Item > { self . raw . next () . map (| it | it . map (SyntaxNode :: from)) } }
    };
}

impl_73!();