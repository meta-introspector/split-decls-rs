macro_rules! CompressedTempfile {
    () => {
        type CompressedTempfile = deflate :: Write < NamedTempFile > ;
    };
}

CompressedTempfile!()