macro_rules! deps {
    () => {
        InternalInternal!();
    };
}

macro_rules! InternalFixed {
    () => {
        deps!();
        # [doc = " An opaque type representing fixed-format item types for internal uses only."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] # [cfg_attr (feature = "defmt" , derive (defmt :: Format))] pub struct InternalFixed { val : InternalInternal , }
    };
}

InternalFixed!();