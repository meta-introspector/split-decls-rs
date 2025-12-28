macro_rules! deps {
    () => {
        VersionFiles!();
    };
}

macro_rules! VersionFileId {
    () => {
        deps!();
        # [doc = " An ID for referring to a filename in [`VersionFiles`]."] # [derive (Clone , Copy , PartialEq , Eq)] pub struct VersionFileId (usize) ;
    };
}

VersionFileId!()