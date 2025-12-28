macro_rules! deps {
    () => {
        Offset!();
        Id!();
    };
}

macro_rules! Entry {
    () => {
        deps!();
        # [doc = " An entry of a chunk file index"] pub struct Entry { # [doc = " The kind of the chunk file"] pub kind : crate :: Id , # [doc = " The offset, relative to the beginning of the file, at which to find the chunk and its end."] pub offset : Range < crate :: file :: Offset > , }
    };
}

Entry!()