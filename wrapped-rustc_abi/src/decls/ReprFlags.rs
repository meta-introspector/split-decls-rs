macro_rules! ReprFlags {
    () => {
        # [derive (Clone , Copy , PartialEq , Eq , Default)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_Generic))] pub struct ReprFlags (u8) ;
    };
}

ReprFlags!();