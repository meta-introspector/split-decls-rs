macro_rules! is_done_ntfs {
    () => {
        # [doc = " Check if trailing filename bytes leave a match to special files like `.git` unchanged in NTFS."] fn is_done_ntfs (input : Option < & [u8] >) -> bool { let Some (input) = input else { return true } ; for b in input . bytes () { if b == b':' { return true ; } if b != b' ' && b != b'.' { return false ; } } true }
    };
}

is_done_ntfs!()