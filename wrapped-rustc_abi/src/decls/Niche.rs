macro_rules! deps {
    () => {
        Size!();
        Primitive!();
        WrappingRange!();
    };
}

macro_rules! Niche {
    () => {
        deps!();
        # [derive (Clone , Copy , PartialEq , Eq , Hash , Debug)] # [cfg_attr (feature = "nightly" , derive (HashStable_Generic))] pub struct Niche { pub offset : Size , pub value : Primitive , pub valid_range : WrappingRange , }
    };
}

Niche!()