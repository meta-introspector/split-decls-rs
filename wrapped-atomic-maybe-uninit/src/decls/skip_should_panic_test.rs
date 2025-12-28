macro_rules! skip_should_panic_test {
    () => {
        fn skip_should_panic_test () -> bool { is_panic_abort () }
    };
}

skip_should_panic_test!()