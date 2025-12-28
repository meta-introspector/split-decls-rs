macro_rules! Edges {
    () => {
        pub type Edges < 'a , E > = Cow < 'a , [E] > ;
    };
}

Edges!();