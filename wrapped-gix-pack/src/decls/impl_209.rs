macro_rules! deps {
    () => {
        File!();
        Kind!();
        Version!();
        EntryIndex!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        # [doc = " Basic file information"] impl File { # [doc = " The version of the pack index"] pub fn version (& self) -> Version { self . version } # [doc = " The path of the opened index file"] pub fn path (& self) -> & std :: path :: Path { & self . path } # [doc = " The amount of objects stored in the pack and index, as one past the highest entry index."] pub fn num_objects (& self) -> EntryIndex { self . num_objects } # [doc = " The kind of hash we assume"] pub fn object_hash (& self) -> gix_hash :: Kind { self . object_hash } }
    };
}

impl_209!();