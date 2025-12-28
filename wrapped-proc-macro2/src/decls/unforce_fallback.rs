macro_rules! unforce_fallback {
    () => {
        pub (crate) fn unforce_fallback () { initialize () ; }
    };
}

unforce_fallback!()