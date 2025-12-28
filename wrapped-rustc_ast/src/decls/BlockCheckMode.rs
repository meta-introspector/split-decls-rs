macro_rules! deps {
    () => {
        UnsafeSource!();
        Walkable!();
    };
}

macro_rules! BlockCheckMode {
    () => {
        deps!();
        # [derive (Clone , PartialEq , Encodable , Decodable , Debug , Copy , Walkable)] pub enum BlockCheckMode { Default , Unsafe (UnsafeSource) , }
    };
}

BlockCheckMode!();