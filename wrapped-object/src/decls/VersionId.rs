macro_rules! deps {
    () => {
        Versions!();
    };
}

macro_rules! VersionId {
    () => {
        deps!();
        # [doc = " An ID for referring to a version in [`Versions`]."] # [derive (Clone , Copy , PartialEq , Eq)] pub struct VersionId (usize) ;
    };
}

VersionId!()