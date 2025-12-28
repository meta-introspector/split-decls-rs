macro_rules! deps {
    () => {
        GeneratedInputStruct!();
        Cycle!();
    };
}

macro_rules! TrackedQuery {
    () => {
        deps!();
        pub (crate) struct TrackedQuery { pub (crate) trait_name : Ident , pub (crate) signature : syn :: Signature , pub (crate) pat_and_tys : Vec < PatType > , pub (crate) invoke : Option < Path > , pub (crate) default : Option < syn :: Block > , pub (crate) cycle : Option < Cycle > , pub (crate) lru : Option < u32 > , pub (crate) generated_struct : Option < GeneratedInputStruct > , }
    };
}

TrackedQuery!();