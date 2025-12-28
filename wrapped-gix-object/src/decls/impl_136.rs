macro_rules! deps {
    () => {
        EntryKind!();
        Tree!();
        Commit!();
        Blob!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        # [doc = " Serialization"] impl EntryKind { # [doc = " Return the representation as used in the git internal format."] pub fn as_octal_str (& self) -> & 'static BStr { use EntryKind :: * ; let bytes : & [u8] = match self { Tree => b"40000" , Blob => b"100644" , BlobExecutable => b"100755" , Link => b"120000" , Commit => b"160000" , } ; bytes . into () } }
    };
}

impl_136!();