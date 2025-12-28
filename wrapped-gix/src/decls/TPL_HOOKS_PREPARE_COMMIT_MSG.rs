macro_rules! TPL_HOOKS_PREPARE_COMMIT_MSG {
    () => {
        const TPL_HOOKS_PREPARE_COMMIT_MSG : & [u8] = include_bytes ! ("assets/init/hooks/prepare-commit-msg.sample") ;
    };
}

TPL_HOOKS_PREPARE_COMMIT_MSG!();