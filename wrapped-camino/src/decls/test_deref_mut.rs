macro_rules! deps {
    () => {
        Utf8PathBuf!();
        Utf8Path!();
    };
}

macro_rules! test_deref_mut {
    () => {
        deps!();
        # [cfg (path_buf_deref_mut)] # [test] fn test_deref_mut () { let mut path_buf = Utf8PathBuf :: from ("foobar") ; let _ : & mut Utf8Path = & mut path_buf ; }
    };
}

test_deref_mut!()