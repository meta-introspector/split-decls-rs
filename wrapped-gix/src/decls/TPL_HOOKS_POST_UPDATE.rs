macro_rules! TPL_HOOKS_POST_UPDATE {
    () => {
        const TPL_HOOKS_POST_UPDATE : & [u8] = include_bytes ! ("assets/init/hooks/post-update.sample") ;
    };
}

TPL_HOOKS_POST_UPDATE!()