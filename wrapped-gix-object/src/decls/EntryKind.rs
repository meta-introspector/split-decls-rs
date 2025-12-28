macro_rules! deps {
    () => {
        Commit!();
        Tree!();
        Blob!();
    };
}

macro_rules! EntryKind {
    () => {
        deps!();
        # [doc = " A discretized version of ideal and valid values for entry modes."] # [doc = ""] # [doc = " Note that even though it can represent every valid [mode](EntryMode), it might"] # [doc = " lose information due to that as well."] # [derive (Clone , Copy , PartialEq , Eq , Debug , Ord , PartialOrd , Hash)] # [repr (u16)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum EntryKind { # [doc = " A tree, or directory"] Tree = 0o040000u16 , # [doc = " A file that is not executable"] Blob = 0o100644 , # [doc = " A file that is executable"] BlobExecutable = 0o100755 , # [doc = " A symbolic link"] Link = 0o120000 , # [doc = " A commit of a git submodule"] Commit = 0o160000 , }
    };
}

EntryKind!();