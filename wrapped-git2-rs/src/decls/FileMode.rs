macro_rules! deps {
    () => {
        Commit!();
        Tree!();
        Blob!();
    };
}

macro_rules! FileMode {
    () => {
        deps!();
        # [doc = " Valid modes for index and tree entries."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum FileMode { # [doc = " Unreadable"] Unreadable , # [doc = " Tree"] Tree , # [doc = " Blob"] Blob , # [doc = " Group writable blob. Obsolete mode kept for compatibility reasons"] BlobGroupWritable , # [doc = " Blob executable"] BlobExecutable , # [doc = " Link"] Link , # [doc = " Commit"] Commit , }
    };
}

FileMode!();