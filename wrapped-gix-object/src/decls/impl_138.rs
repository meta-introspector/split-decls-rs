macro_rules! deps {
    () => {
        EntryMode!();
        Commit!();
        Tree!();
        Blob!();
        EntryKind!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        impl EntryMode { # [doc = " Discretize the raw mode into an enum with well-known state while dropping unnecessary details."] pub const fn kind (& self) -> EntryKind { let etype = self . value () & IFMT ; if etype == 0o100000 { if self . value () & 0o000100 == 0o000100 { EntryKind :: BlobExecutable } else { EntryKind :: Blob } } else if etype == EntryKind :: Link as u16 { EntryKind :: Link } else if etype == EntryKind :: Tree as u16 { EntryKind :: Tree } else { EntryKind :: Commit } } # [doc = " Return true if this entry mode represents a Tree/directory"] pub const fn is_tree (& self) -> bool { self . value () & IFMT == EntryKind :: Tree as u16 } # [doc = " Return true if this entry mode represents the commit of a submodule."] pub const fn is_commit (& self) -> bool { self . value () & IFMT == EntryKind :: Commit as u16 } # [doc = " Return true if this entry mode represents a symbolic link"] pub const fn is_link (& self) -> bool { self . value () & IFMT == EntryKind :: Link as u16 } # [doc = " Return true if this entry mode represents anything BUT Tree/directory"] pub const fn is_no_tree (& self) -> bool { self . value () & IFMT != EntryKind :: Tree as u16 } # [doc = " Return true if the entry is any kind of blob."] pub const fn is_blob (& self) -> bool { self . value () & IFMT == 0o100000 } # [doc = " Return true if the entry is an executable blob."] pub const fn is_executable (& self) -> bool { matches ! (self . kind () , EntryKind :: BlobExecutable) } # [doc = " Return true if the entry is any kind of blob or symlink."] pub const fn is_blob_or_symlink (& self) -> bool { matches ! (self . kind () , EntryKind :: Blob | EntryKind :: BlobExecutable | EntryKind :: Link) } # [doc = " Represent the mode as descriptive string."] pub const fn as_str (& self) -> & 'static str { use EntryKind :: * ; match self . kind () { Tree => "tree" , Blob => "blob" , BlobExecutable => "exe" , Link => "link" , Commit => "commit" , } } }
    };
}

impl_138!();