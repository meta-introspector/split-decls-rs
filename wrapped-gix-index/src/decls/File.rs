macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! File {
    () => {
        deps!();
        # [doc = " An index file whose state was read from a file on disk."] # [derive (Clone)] pub struct File { # [doc = " The state containing the actual index data."] pub (crate) state : State , # [doc = " The path from which the index was read or to which it is supposed to be written."] pub (crate) path : PathBuf , # [doc = " The checksum of all bytes prior to the checksum itself."] pub (crate) checksum : Option < gix_hash :: ObjectId > , }
    };
}

File!();