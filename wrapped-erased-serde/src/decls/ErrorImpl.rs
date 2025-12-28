macro_rules! ErrorImpl {
    () => {
        pub enum ErrorImpl { ShortCircuit , Custom (Box < String >) , }
    };
}

ErrorImpl!()