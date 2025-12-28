macro_rules! ReleaseChannel {
    () => {
        # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash , PartialOrd , Ord)] pub enum ReleaseChannel { Stable , Beta , Nightly , }
    };
}

ReleaseChannel!()