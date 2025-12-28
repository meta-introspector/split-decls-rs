macro_rules! assert_future {
    () => {
        pub (crate) fn assert_future < T , F > (future : F) -> F where F : Future < Output = T > , { future }
    };
}

assert_future!()