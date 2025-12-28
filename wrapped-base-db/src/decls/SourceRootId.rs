macro_rules! SourceRootId {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , PartialOrd , Ord)] pub struct SourceRootId (pub u32) ;
    };
}

SourceRootId!()