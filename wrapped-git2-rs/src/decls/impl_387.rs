macro_rules! deps {
    () => {
        IndexEntry!();
        Error!();
    };
}

macro_rules! impl_387 {
    () => {
        deps!();
        impl IndexEntry { # [doc = " Create a raw index entry."] # [doc = ""] # [doc = " The returned `raw::git_index_entry` contains a pointer to a `CString` path, which is also"] # [doc = " returned because it's lifetime must exceed the lifetime of the `raw::git_index_entry`."] pub (crate) unsafe fn to_raw (& self) -> Result < (raw :: git_index_entry , CString) , Error > { let path = CString :: new (& self . path [..]) ? ; let mut flags = self . flags & ! raw :: GIT_INDEX_ENTRY_NAMEMASK ; if self . path . len () < raw :: GIT_INDEX_ENTRY_NAMEMASK as usize { flags |= self . path . len () as u16 ; } else { flags |= raw :: GIT_INDEX_ENTRY_NAMEMASK ; } unsafe { let raw = raw :: git_index_entry { dev : self . dev , ino : self . ino , mode : self . mode , uid : self . uid , gid : self . gid , file_size : self . file_size , id : * self . id . raw () , flags , flags_extended : self . flags_extended , path : path . as_ptr () , mtime : raw :: git_index_time { seconds : self . mtime . seconds () , nanoseconds : self . mtime . nanoseconds () , } , ctime : raw :: git_index_time { seconds : self . ctime . seconds () , nanoseconds : self . ctime . nanoseconds () , } , } ; Ok ((raw , path)) } } }
    };
}

impl_387!();