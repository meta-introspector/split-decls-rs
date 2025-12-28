macro_rules! ExpectedPointerMutability {
    () => {
        pub enum ExpectedPointerMutability { Mut , Not , }
    };
}

ExpectedPointerMutability!()