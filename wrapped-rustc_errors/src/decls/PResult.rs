macro_rules! PResult {
    () => {
        pub type PResult < 'a , T > = Result < T , Diag < 'a > > ;
    };
}

PResult!()