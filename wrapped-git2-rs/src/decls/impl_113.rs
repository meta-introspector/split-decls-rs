macro_rules! deps {
    () => {
        FileMode!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl From < FileMode > for i32 { fn from (mode : FileMode) -> i32 { match mode { FileMode :: Unreadable => raw :: GIT_FILEMODE_UNREADABLE as i32 , FileMode :: Tree => raw :: GIT_FILEMODE_TREE as i32 , FileMode :: Blob => raw :: GIT_FILEMODE_BLOB as i32 , FileMode :: BlobGroupWritable => raw :: GIT_FILEMODE_BLOB_GROUP_WRITABLE as i32 , FileMode :: BlobExecutable => raw :: GIT_FILEMODE_BLOB_EXECUTABLE as i32 , FileMode :: Link => raw :: GIT_FILEMODE_LINK as i32 , FileMode :: Commit => raw :: GIT_FILEMODE_COMMIT as i32 , } } }
    };
}

impl_113!()