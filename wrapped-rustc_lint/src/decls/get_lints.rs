macro_rules! get_lints {
    () => {
        pub (crate) fn get_lints () -> LintVec { vec ! [CLASHING_EXTERN_DECLARATIONS] }
    };
}

get_lints!()