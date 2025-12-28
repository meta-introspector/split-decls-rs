macro_rules! h {
    () => {
        fn h < D : Digest > (mut d : D , items : & [& [u8]]) -> String { for i in items { d . update (i) ; } hex :: encode (d . finalize ()) }
    };
}

h!();