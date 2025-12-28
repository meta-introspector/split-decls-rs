macro_rules! Nodes {
    () => {
        pub type Nodes < 'a , N > = Cow < 'a , [N] > ;
    };
}

Nodes!()