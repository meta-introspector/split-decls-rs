macro_rules! deps {
    () => {
        IndexTime!();
        Note!();
        Oid!();
    };
}

macro_rules! IndexEntry {
    () => {
        deps!();
        # [doc = " A structure to represent an entry or a file inside of an index."] # [doc = ""] # [doc = " All fields of an entry are public for modification and inspection. This is"] # [doc = " also how a new index entry is created."] # [allow (missing_docs)] # [derive (Debug)] pub struct IndexEntry { pub ctime : IndexTime , pub mtime : IndexTime , pub dev : u32 , pub ino : u32 , pub mode : u32 , pub uid : u32 , pub gid : u32 , pub file_size : u32 , pub id : Oid , pub flags : u16 , pub flags_extended : u16 , # [doc = " The path of this index entry as a byte vector. Regardless of the"] # [doc = " current platform, the directory separator is an ASCII forward slash"] # [doc = " (`0x2F`). There are no terminating or internal NUL characters, and no"] # [doc = " trailing slashes. Most of the time, paths will be valid utf-8 — but"] # [doc = " not always. For more information on the path storage format, see"] # [doc = " [these git docs][git-index-docs]. Note that libgit2 will take care of"] # [doc = " handling the prefix compression mentioned there."] # [doc = ""] # [doc = " [git-index-docs]: https://github.com/git/git/blob/a08a83db2bf27f015bec9a435f6d73e223c21c5e/Documentation/technical/index-format.txt#L107-L124"] # [doc = ""] # [doc = " You can turn this value into a `std::ffi::CString` with"] # [doc = " `CString::new(&entry.path[..]).unwrap()`. To turn a reference into a"] # [doc = " `&std::path::Path`, see the `bytes2path()` function in the private,"] # [doc = " internal `util` module in this crate’s source code."] pub path : Vec < u8 > , }
    };
}

IndexEntry!();