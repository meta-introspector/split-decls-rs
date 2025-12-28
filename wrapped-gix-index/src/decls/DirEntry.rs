macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! DirEntry {
    () => {
        deps!();
        struct DirEntry < 'a > { # [doc = " The first entry in the directory"] entry : & 'a Entry , # [doc = " One past the last byte of the directory in the path-backing"] dir_end : usize , }
    };
}

DirEntry!()