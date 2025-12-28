macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! content {
    () => {
        deps!();
        pub (crate) fn content (input : & [u8]) -> Result < Vec < PathBuf > , Error > { let mut out = Vec :: new () ; for line in input . split (| b | * b == b'\n') { let line = line . as_bstr () ; if line . is_empty () || line . starts_with (b"#") { continue ; } out . push (gix_path :: try_from_bstr (if line . starts_with (b"\"") { gix_quote :: ansi_c :: undo (line) ? . 0 } else { Cow :: Borrowed (line) }) . map_err (| _ | Error :: PathConversion (line . to_vec ())) ? . into_owned () ,) ; } Ok (out) }
    };
}

content!();