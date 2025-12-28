macro_rules! macro_907 {
    () => {
        bitflags ! { # [allow (missing_docs)] # [derive (Clone , Copy , Debug , Eq , PartialEq , PartialOrd , Ord , Hash)] pub struct AttrCheckFlags : u32 { # [doc = " Check the working directory, then the index."] const FILE_THEN_INDEX = raw :: GIT_ATTR_CHECK_FILE_THEN_INDEX as u32 ; # [doc = " Check the index, then the working directory."] const INDEX_THEN_FILE = raw :: GIT_ATTR_CHECK_INDEX_THEN_FILE as u32 ; # [doc = " Check the index only."] const INDEX_ONLY = raw :: GIT_ATTR_CHECK_INDEX_ONLY as u32 ; # [doc = " Do not use the system gitattributes file."] const NO_SYSTEM = raw :: GIT_ATTR_CHECK_NO_SYSTEM as u32 ; } }
    };
}

macro_907!()