macro_rules! deps {
    () => {
        UnsafeSource!();
    };
}

macro_rules! BlockCheckMode {
    () => {
        deps!();
        # [derive (Copy , Clone , PartialEq , Debug , HashStable_Generic)] pub enum BlockCheckMode { DefaultBlock , UnsafeBlock (UnsafeSource) , }
    };
}

BlockCheckMode!();