macro_rules! mtime {
    () => {
        # [doc = " Returns the last modification time of a file."] pub fn mtime (path : & Path) -> Result < FileTime > { let meta = metadata (path) ? ; Ok (FileTime :: from_last_modification_time (& meta)) }
    };
}

mtime!()