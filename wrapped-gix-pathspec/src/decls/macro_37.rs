macro_rules! macro_37 {
    () => {
        bitflags ! { # [doc = " Flags to represent 'magic signatures' which are parsed behind colons, like `:top:`."] # [derive (Default , PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] pub struct MagicSignature : u32 { # [doc = " Matches patterns from the root of the repository"] const TOP = 1 << 0 ; # [doc = " Matches patterns in case insensitive mode"] const ICASE = 1 << 1 ; # [doc = " Excludes the matching patterns from the previous results"] const EXCLUDE = 1 << 2 ; # [doc = " The pattern must match a directory, and not a file."] # [doc = " This is equivalent to how it's handled in `gix-glob`"] const MUST_BE_DIR = 1 << 3 ; } }
    };
}

macro_37!();