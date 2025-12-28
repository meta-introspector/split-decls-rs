macro_rules! deps {
    () => {
        Integer!();
    };
}

macro_rules! IntegerType {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_Generic))] pub enum IntegerType { # [doc = " Pointer-sized integer type, i.e. `isize` and `usize`. The field shows signedness, e.g."] # [doc = " `Pointer(true)` means `isize`."] Pointer (bool) , # [doc = " Fixed-sized integer type, e.g. `i8`, `u32`, `i128`. The bool field shows signedness, e.g."] # [doc = " `Fixed(I8, false)` means `u8`."] Fixed (Integer , bool) , }
    };
}

IntegerType!();