macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! test_owned_into {
    () => {
        deps!();
        # [test] fn test_owned_into () { let utf8_path_buf = Utf8PathBuf :: from ("test/path") ; all_into ! (Utf8PathBuf , utf8_path_buf) ; }
    };
}

test_owned_into!();