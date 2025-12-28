macro_rules! CompareWithoutTsFn {
    () => {
        pub type CompareWithoutTsFn = dyn Fn (& [u8] , bool , & [u8] , bool) -> Ordering ;
    };
}

CompareWithoutTsFn!();