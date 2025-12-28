macro_rules! Noop {
    () => {
        pub (crate) struct Noop ;
    };
}

Noop!()