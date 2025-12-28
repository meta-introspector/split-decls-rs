macro_rules! TPL_HOOKS_PRE_APPLYPATCH {
    () => {
        const TPL_HOOKS_PRE_APPLYPATCH : & [u8] = include_bytes ! ("assets/init/hooks/pre-applypatch.sample") ;
    };
}

TPL_HOOKS_PRE_APPLYPATCH!()