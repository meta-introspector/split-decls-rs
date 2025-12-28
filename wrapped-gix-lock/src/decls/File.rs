macro_rules! File {
    () => {
        # [doc = " Locks a resource to eventually be overwritten with the content of this file."] # [doc = ""] # [doc = " Dropping the file without [committing][File::commit] will delete it, leaving the underlying resource unchanged."] # [must_use = "A File that is immediately dropped doesn't allow resource updates"] # [derive (Debug)] pub struct File { inner : gix_tempfile :: Handle < Writable > , lock_path : PathBuf , }
    };
}

File!();