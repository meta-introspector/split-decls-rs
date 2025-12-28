macro_rules! deps {
    () => {
        LineProgram!();
    };
}

macro_rules! id {
    () => {
        deps!();
        mod id { # [doc = " An identifier for a file in a `LineProgram`."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct FileId (usize) ; impl FileId { # [doc = " Create a `FileId` given a 0-based index into `LineProgram::files`."] pub (crate) fn new (index : usize) -> Self { FileId (index) } # [doc = " The 0-based index of the file in `LineProgram::files`."] pub (super) fn index (self) -> usize { self . 0 } # [doc = " The initial state of the file register."] pub (super) fn initial_state (version : u16) -> Self { if version == 5 { FileId (1) } else { FileId (0) } } # [doc = " Convert to a raw value used for writing."] # [doc = ""] # [doc = " This converts to a 1-based index for DWARF version <= 4."] pub (crate) fn raw (self , version : u16) -> u64 { if version <= 4 { self . 0 as u64 + 1 } else { self . 0 as u64 } } } }
    };
}

id!()