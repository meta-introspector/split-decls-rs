macro_rules! LintStoreExpandImpl {
    () => {
        struct LintStoreExpandImpl < 'a > (& 'a LintStore) ;
    };
}

LintStoreExpandImpl!()