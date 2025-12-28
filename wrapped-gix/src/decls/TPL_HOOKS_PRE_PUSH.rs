macro_rules! TPL_HOOKS_PRE_PUSH {
    () => {
        const TPL_HOOKS_PRE_PUSH : & [u8] = include_bytes ! ("assets/init/hooks/pre-push.sample") ;
    };
}

TPL_HOOKS_PRE_PUSH!();