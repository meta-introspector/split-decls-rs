macro_rules! macro_7 {
    () => {
        bitflags :: bitflags ! { # [doc = " Possible file type filters."] # [doc = " Constants can be OR'd to filter for several types at a time."] # [doc = ""] # [doc = " Note that not all files are represented in this enum."] # [doc = " For example, a char-device is neither a file, a directory, nor a symlink."] pub struct FileType : u32 { # [allow (missing_docs)] const FILE = 0b001 ; # [allow (missing_docs)] const DIR = 0b010 ; # [allow (missing_docs)] const SYMLINK = 0b100 ; } }
    };
}

macro_7!()