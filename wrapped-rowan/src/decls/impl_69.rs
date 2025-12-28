macro_rules! deps {
    () => {
        NodeOrToken!();
        Language!();
        SyntaxElementChildren!();
        SyntaxElement!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl < L : Language > Iterator for SyntaxElementChildren < L > { type Item = SyntaxElement < L > ; fn next (& mut self) -> Option < Self :: Item > { self . raw . next () . map (NodeOrToken :: from) } }
    };
}

impl_69!()