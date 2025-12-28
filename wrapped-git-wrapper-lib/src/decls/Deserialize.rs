macro_rules! Deserialize {
    () => {
        # [cfg (not (feature = "serde_enabled"))] # [derive (Debug , Clone , PartialEq , Eq)] pub struct Deserialize ;
    };
}

Deserialize!()