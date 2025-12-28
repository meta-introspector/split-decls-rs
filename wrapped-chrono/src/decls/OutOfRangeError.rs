macro_rules! deps {
    () => {
        TimeDelta!();
    };
}

macro_rules! OutOfRangeError {
    () => {
        deps!();
        # [doc = " Represents error when converting `TimeDelta` to/from a standard library"] # [doc = " implementation"] # [doc = ""] # [doc = " The `std::time::Duration` supports a range from zero to `u64::MAX`"] # [doc = " *seconds*, while this module supports signed range of up to"] # [doc = " `i64::MAX` of *milliseconds*."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] # [cfg_attr (feature = "defmt" , derive (defmt :: Format))] pub struct OutOfRangeError (()) ;
    };
}

OutOfRangeError!()