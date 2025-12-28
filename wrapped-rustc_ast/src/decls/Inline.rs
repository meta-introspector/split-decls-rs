macro_rules! deps {
    () => {
        Walkable!();
    };
}

macro_rules! Inline {
    () => {
        deps!();
        # [derive (Clone , Copy , PartialEq , Encodable , Decodable , Debug , Walkable)] pub enum Inline { Yes , No { had_parse_error : Result < () , ErrorGuaranteed > } , }
    };
}

Inline!()