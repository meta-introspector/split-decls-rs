macro_rules! Cancellable {
    () => {
        pub type Cancellable < T > = Result < T , Cancelled > ;
    };
}

Cancellable!();