macro_rules! deps {
    () => {
        IndexEntry!();
        Binding!();
    };
}

macro_rules! impl_395 {
    () => {
        deps!();
        impl Binding for IndexEntry { type Raw = raw :: git_index_entry ; unsafe fn from_raw (raw : raw :: git_index_entry) -> IndexEntry { let raw :: git_index_entry { ctime , mtime , dev , ino , mode , uid , gid , file_size , id , flags , flags_extended , path , } = raw ; let mut pathlen = (flags & raw :: GIT_INDEX_ENTRY_NAMEMASK) as usize ; if pathlen == raw :: GIT_INDEX_ENTRY_NAMEMASK as usize { pathlen = CStr :: from_ptr (path) . to_bytes () . len () ; } let path = slice :: from_raw_parts (path as * const u8 , pathlen) ; IndexEntry { dev , ino , mode , uid , gid , file_size , id : Binding :: from_raw (& id as * const _) , flags , flags_extended , path : path . to_vec () , mtime : Binding :: from_raw (mtime) , ctime : Binding :: from_raw (ctime) , } } fn raw (& self) -> raw :: git_index_entry { panic ! () } }
    };
}

impl_395!()