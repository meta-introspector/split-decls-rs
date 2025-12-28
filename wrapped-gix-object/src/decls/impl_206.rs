macro_rules! deps {
    () => {
        Tree!();
        Commit!();
        Tag!();
        Error!();
        Blob!();
        Kind!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        # [doc = " Initialization"] impl Kind { # [doc = " Parse a `Kind` from its serialized loose git objects."] pub fn from_bytes (s : & [u8]) -> Result < Kind , Error > { Ok (match s { b"tree" => Kind :: Tree , b"blob" => Kind :: Blob , b"commit" => Kind :: Commit , b"tag" => Kind :: Tag , _ => return Err (Error :: InvalidObjectKind { kind : s . into () }) , }) } }
    };
}

impl_206!();