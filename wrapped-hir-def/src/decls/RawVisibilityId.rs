macro_rules! RawVisibilityId {
    () => {
        # [derive (Copy , Clone , Eq , PartialEq)] pub (crate) struct RawVisibilityId (u32) ;
    };
}

RawVisibilityId!()