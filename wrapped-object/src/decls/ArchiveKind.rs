macro_rules! ArchiveKind {
    () => {
        # [doc = " The kind of archive format."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [non_exhaustive] pub enum ArchiveKind { # [doc = " There are no special files that indicate the archive format."] Unknown , # [doc = " The GNU (or System V) archive format."] Gnu , # [doc = " The GNU (or System V) archive format with 64-bit symbol table."] Gnu64 , # [doc = " The BSD archive format."] Bsd , # [doc = " The BSD archive format with 64-bit symbol table."] # [doc = ""] # [doc = " This is used for Darwin."] Bsd64 , # [doc = " The Windows COFF archive format."] Coff , # [doc = " The AIX big archive format."] AixBig , }
    };
}

ArchiveKind!()