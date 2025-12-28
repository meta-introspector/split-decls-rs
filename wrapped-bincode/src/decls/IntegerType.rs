macro_rules! deps {
    () => {
        DecodeError!();
    };
}

macro_rules! IntegerType {
    () => {
        deps!();
        # [doc = " Integer types. Used by [DecodeError]. These types have no purpose other than being shown in errors."] # [non_exhaustive] # [derive (Debug , PartialEq , Eq)] # [allow (missing_docs)] pub enum IntegerType { U8 , U16 , U32 , U64 , U128 , Usize , I8 , I16 , I32 , I64 , I128 , Isize , Reserved , }
    };
}

IntegerType!();