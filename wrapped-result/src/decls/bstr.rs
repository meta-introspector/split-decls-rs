macro_rules! bstr {
    () => {
        # [cfg (all (windows , not (windows_slim_errors)))] mod bstr ;
    };
}

bstr!()