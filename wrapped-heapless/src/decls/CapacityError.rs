macro_rules! deps {
    () => {
        String!();
        Vec!();
    };
}

macro_rules! CapacityError {
    () => {
        deps!();
        # [doc = " The error type for fallible [`Vec`] and [`String`] methods."] # [derive (Debug , Default)] # [non_exhaustive] pub struct CapacityError ;
    };
}

CapacityError!()