macro_rules! DummyWaker {
    () => {
        pub (crate) struct DummyWaker () ;
    };
}

DummyWaker!();