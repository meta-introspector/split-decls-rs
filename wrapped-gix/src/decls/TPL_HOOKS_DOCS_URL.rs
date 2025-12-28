macro_rules! TPL_HOOKS_DOCS_URL {
    () => {
        const TPL_HOOKS_DOCS_URL : & [u8] = include_bytes ! ("assets/init/hooks/docs.url") ;
    };
}

TPL_HOOKS_DOCS_URL!();