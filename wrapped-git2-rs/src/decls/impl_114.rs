macro_rules! deps {
    () => {
        FileMode!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl From < FileMode > for u32 { fn from (mode : FileMode) -> u32 { match mode { FileMode :: Unreadable => raw :: GIT_FILEMODE_UNREADABLE as u32 , FileMode :: Tree => raw :: GIT_FILEMODE_TREE as u32 , FileMode :: Blob => raw :: GIT_FILEMODE_BLOB as u32 , FileMode :: BlobGroupWritable => raw :: GIT_FILEMODE_BLOB_GROUP_WRITABLE as u32 , FileMode :: BlobExecutable => raw :: GIT_FILEMODE_BLOB_EXECUTABLE as u32 , FileMode :: Link => raw :: GIT_FILEMODE_LINK as u32 , FileMode :: Commit => raw :: GIT_FILEMODE_COMMIT as u32 , } } }
    };
}

impl_114!()