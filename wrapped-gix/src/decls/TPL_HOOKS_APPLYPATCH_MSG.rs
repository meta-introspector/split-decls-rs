macro_rules! TPL_HOOKS_APPLYPATCH_MSG {
    () => {
        const TPL_HOOKS_APPLYPATCH_MSG : & [u8] = include_bytes ! ("assets/init/hooks/applypatch-msg.sample") ;
    };
}

TPL_HOOKS_APPLYPATCH_MSG!()