macro_rules! PotentialUtf16 {
    () => {
        # [repr (transparent)] # [derive (PartialEq , Eq , PartialOrd , Ord)] # [allow (clippy :: exhaustive_structs)] pub struct PotentialUtf16 (pub [u16]) ;
    };
}

PotentialUtf16!();