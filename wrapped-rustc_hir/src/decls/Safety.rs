macro_rules! Safety {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] # [derive (Encodable , Decodable , HashStable_Generic)] pub enum Safety { Unsafe , Safe , }
    };
}

Safety!();