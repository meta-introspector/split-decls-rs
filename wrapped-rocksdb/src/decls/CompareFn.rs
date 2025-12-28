macro_rules! CompareFn {
    () => {
        pub type CompareFn = dyn Fn (& [u8] , & [u8]) -> Ordering ;
    };
}

CompareFn!()