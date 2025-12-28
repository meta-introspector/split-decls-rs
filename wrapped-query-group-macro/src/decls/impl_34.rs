macro_rules! deps {
    () => {
        Parenthesized!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < T > syn :: parse :: Parse for Parenthesized < T > where T : syn :: parse :: Parse , { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { let content ; syn :: parenthesized ! (content in input) ; content . parse :: < T > () . map (Parenthesized) } }
    };
}

impl_34!();