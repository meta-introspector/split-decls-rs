macro_rules! deps {
    () => {
        AllocKind!();
    };
}

macro_rules! AllocInfo {
    () => {
        deps!();
        # [doc = " Metadata about an `AllocId`."] # [derive (Copy , Clone , PartialEq , Debug)] pub struct AllocInfo { pub size : Size , pub align : Align , pub kind : AllocKind , pub mutbl : Mutability , }
    };
}

AllocInfo!();