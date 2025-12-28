macro_rules! Serialize {
    () => {
        # [cfg (not (feature = "serde_enabled"))] # [derive (Debug , Clone , PartialEq , Eq)] pub struct Serialize ;
    };
}

Serialize!()