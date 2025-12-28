macro_rules! deps {
    () => {
        PrintAttribute!();
    };
}

macro_rules! MirDialect {
    () => {
        deps!();
        # [derive (Clone , Copy , Decodable , Debug , Encodable , PartialEq)] # [derive (HashStable_Generic , PrintAttribute)] pub enum MirDialect { Analysis , Built , Runtime , }
    };
}

MirDialect!()