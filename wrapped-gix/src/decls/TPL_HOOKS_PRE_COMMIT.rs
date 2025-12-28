macro_rules! TPL_HOOKS_PRE_COMMIT {
    () => {
        const TPL_HOOKS_PRE_COMMIT : & [u8] = include_bytes ! ("assets/init/hooks/pre-commit.sample") ;
    };
}

TPL_HOOKS_PRE_COMMIT!()