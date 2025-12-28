macro_rules! IoReader {
    () => {
        pub (crate) struct IoReader < R > { reader : R , }
    };
}

IoReader!()