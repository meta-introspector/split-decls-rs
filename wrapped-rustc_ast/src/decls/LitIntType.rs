macro_rules! deps {
    () => {
        Type!();
    };
}

macro_rules! LitIntType {
    () => {
        deps!();
        # [doc = " Type of the integer literal based on provided suffix."] # [derive (Clone , Copy , Encodable , Decodable , Debug , Hash , Eq , PartialEq)] # [derive (HashStable_Generic)] pub enum LitIntType { # [doc = " e.g. `42_i32`."] Signed (IntTy) , # [doc = " e.g. `42_u32`."] Unsigned (UintTy) , # [doc = " e.g. `42`."] Unsuffixed , }
    };
}

LitIntType!();