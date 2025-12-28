macro_rules! TPL_HOOKS_COMMIT_MSG {
    () => {
        const TPL_HOOKS_COMMIT_MSG : & [u8] = include_bytes ! ("assets/init/hooks/commit-msg.sample") ;
    };
}

TPL_HOOKS_COMMIT_MSG!();