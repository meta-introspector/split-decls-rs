macro_rules! deps {
    () => {
        Primitive!();
        WrappingRange!();
    };
}

macro_rules! Scalar {
    () => {
        deps!();
        # [doc = " Information about one scalar component of a Rust type."] # [derive (Clone , Copy , PartialEq , Eq , Hash , Debug)] # [cfg_attr (feature = "nightly" , derive (HashStable_Generic))] pub enum Scalar { Initialized { value : Primitive , valid_range : WrappingRange , } , Union { # [doc = " Even for unions, we need to use the correct registers for the kind of"] # [doc = " values inside the union, so we keep the `Primitive` type around. We"] # [doc = " also use it to compute the size of the scalar."] # [doc = " However, unions never have niches and even allow undef,"] # [doc = " so there is no `valid_range`."] value : Primitive , } , }
    };
}

Scalar!();