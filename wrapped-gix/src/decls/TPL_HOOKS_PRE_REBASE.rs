macro_rules! TPL_HOOKS_PRE_REBASE {
    () => {
        const TPL_HOOKS_PRE_REBASE : & [u8] = include_bytes ! ("assets/init/hooks/pre-rebase.sample") ;
    };
}

TPL_HOOKS_PRE_REBASE!();