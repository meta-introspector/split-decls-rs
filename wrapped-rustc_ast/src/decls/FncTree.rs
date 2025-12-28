macro_rules! deps {
    () => {
        TypeTree!();
    };
}

macro_rules! FncTree {
    () => {
        deps!();
        # [derive (Clone , Eq , PartialEq , Encodable , Decodable , Debug , HashStable_Generic)] pub struct FncTree { pub args : Vec < TypeTree > , pub ret : TypeTree , }
    };
}

FncTree!()