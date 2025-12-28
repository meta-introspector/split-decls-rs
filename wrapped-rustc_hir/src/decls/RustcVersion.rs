macro_rules! deps {
    () => {
        PrintAttribute!();
    };
}

macro_rules! RustcVersion {
    () => {
        deps!();
        # [derive (Encodable , Decodable , Copy , Clone , Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] # [derive (HashStable_Generic , PrintAttribute)] pub struct RustcVersion { pub major : u16 , pub minor : u16 , pub patch : u16 , }
    };
}

RustcVersion!()