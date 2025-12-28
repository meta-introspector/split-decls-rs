macro_rules! deps {
    () => {
        FilesetEntryCommand!();
    };
}

macro_rules! LC_FILESET_ENTRY {
    () => {
        deps!();
        # [doc = " used with `FilesetEntryCommand`"] pub const LC_FILESET_ENTRY : u32 = 0x35 | LC_REQ_DYLD ;
    };
}

LC_FILESET_ENTRY!()