macro_rules! Pinnedness {
    () => {
        # [derive (Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug , Copy)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_NoContext))] pub enum Pinnedness { Not , Pinned , }
    };
}

Pinnedness!();