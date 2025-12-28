macro_rules! deps {
    () => {
        Limb!();
    };
}

macro_rules! Bigint {
    () => {
        deps!();
        # [doc = " Storage for a big integer type."] # [derive (Clone , PartialEq , Eq)] pub (crate) struct Bigint { # [doc = " Internal storage for the Bigint, in little-endian order."] pub (crate) data : Vec < Limb > , }
    };
}

Bigint!()