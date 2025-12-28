macro_rules! deps {
    () => {
        Source!();
    };
}

macro_rules! Metadata {
    () => {
        deps!();
        # [doc = " Additional information about a section."] # [derive (Clone , Debug , PartialOrd , PartialEq , Ord , Eq , Hash)] pub struct Metadata { # [doc = " The file path of the source, if known."] pub path : Option < PathBuf > , # [doc = " Where the section is coming from."] pub source : crate :: Source , # [doc = " The levels of indirection of the file, with 0 being a section"] # [doc = " that was directly loaded, and 1 being an `include.path` of a"] # [doc = " level 0 file."] pub level : u8 , # [doc = " The trust-level for the section this meta-data is associated with."] pub trust : gix_sec :: Trust , }
    };
}

Metadata!()