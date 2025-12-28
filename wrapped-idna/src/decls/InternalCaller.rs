macro_rules! InternalCaller {
    () => {
        pub (crate) struct InternalCaller ;
    };
}

InternalCaller!();