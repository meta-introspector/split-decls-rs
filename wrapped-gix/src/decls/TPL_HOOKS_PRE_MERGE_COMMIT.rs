macro_rules! TPL_HOOKS_PRE_MERGE_COMMIT {
    () => {
        const TPL_HOOKS_PRE_MERGE_COMMIT : & [u8] = include_bytes ! ("assets/init/hooks/pre-merge-commit.sample") ;
    };
}

TPL_HOOKS_PRE_MERGE_COMMIT!();