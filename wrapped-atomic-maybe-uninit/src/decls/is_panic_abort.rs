macro_rules! is_panic_abort {
    () => {
        fn is_panic_abort () -> bool { ! cfg ! (panic = "unwind") }
    };
}

is_panic_abort!()