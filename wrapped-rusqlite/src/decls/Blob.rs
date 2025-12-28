macro_rules! deps {
    () => {
        Connection!();
    };
}

macro_rules! Blob {
    () => {
        deps!();
        # [doc = " Handle to an open BLOB. See"] # [doc = " [`rusqlite::blob`](crate::blob) documentation for in-depth discussion."] pub struct Blob < 'conn > { conn : & 'conn Connection , blob : * mut ffi :: sqlite3_blob , pos : i32 , }
    };
}

Blob!()