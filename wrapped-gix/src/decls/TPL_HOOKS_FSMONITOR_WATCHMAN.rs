macro_rules! TPL_HOOKS_FSMONITOR_WATCHMAN {
    () => {
        const TPL_HOOKS_FSMONITOR_WATCHMAN : & [u8] = include_bytes ! ("assets/init/hooks/fsmonitor-watchman.sample") ;
    };
}

TPL_HOOKS_FSMONITOR_WATCHMAN!()