macro_rules! make_nonnull {
    () => {
        # [cold] fn make_nonnull (v : & str) -> String { v . replace ('\0' , NUL_REPLACE) }
    };
}

make_nonnull!()