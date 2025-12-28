macro_rules! deps {
    () => {
        Walkable!();
    };
}

macro_rules! Recovered {
    () => {
        deps!();
        # [doc = " Was parsing recovery performed?"] # [derive (Copy , Clone , Debug , Encodable , Decodable , HashStable_Generic , Walkable)] pub enum Recovered { No , Yes (ErrorGuaranteed) , }
    };
}

Recovered!()