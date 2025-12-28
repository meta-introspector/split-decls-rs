macro_rules! deps {
    () => {
        Walkable!();
    };
}

macro_rules! UnsafeSource {
    () => {
        deps!();
        # [derive (Clone , PartialEq , Encodable , Decodable , Debug , Copy , Walkable)] pub enum UnsafeSource { CompilerGenerated , UserProvided , }
    };
}

UnsafeSource!();