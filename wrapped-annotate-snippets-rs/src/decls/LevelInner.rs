macro_rules! LevelInner {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq , Eq , PartialOrd , Ord)] pub (crate) enum LevelInner { Error , Warning , Info , Note , Help , }
    };
}

LevelInner!()