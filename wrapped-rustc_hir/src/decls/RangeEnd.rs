macro_rules! RangeEnd {
    () => {
        # [derive (Copy , Clone , PartialEq , Debug , HashStable_Generic , Hash , Eq , Encodable , Decodable)] pub enum RangeEnd { Included , Excluded , }
    };
}

RangeEnd!();