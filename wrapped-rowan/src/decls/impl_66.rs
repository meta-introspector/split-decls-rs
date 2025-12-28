macro_rules! deps {
    () => {
        SyntaxNodeChildren!();
        Language!();
        SyntaxNode!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl < L : Language > Iterator for SyntaxNodeChildren < L > { type Item = SyntaxNode < L > ; fn next (& mut self) -> Option < Self :: Item > { self . raw . next () . map (SyntaxNode :: from) } }
    };
}

impl_66!()