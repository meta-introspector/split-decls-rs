macro_rules! deps {
    () => {
        SyntaxElement!();
        PreorderWithTokens!();
        Language!();
        WalkEvent!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < L : Language > Iterator for PreorderWithTokens < L > { type Item = WalkEvent < SyntaxElement < L > > ; fn next (& mut self) -> Option < Self :: Item > { self . raw . next () . map (| it | it . map (SyntaxElement :: from)) } }
    };
}

impl_76!()