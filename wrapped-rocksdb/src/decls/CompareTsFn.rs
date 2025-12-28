macro_rules! CompareTsFn {
    () => {
        pub type CompareTsFn = dyn Fn (& [u8] , & [u8]) -> Ordering ;
    };
}

CompareTsFn!()