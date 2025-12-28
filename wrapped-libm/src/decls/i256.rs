macro_rules! i256 {
    () => {
        # [doc = " A 256-bit signed integer represented as two 128-bit native-endian limbs."] # [allow (non_camel_case_types)] # [derive (Clone , Copy , Debug , PartialEq , PartialOrd , Eq , Ord)] pub struct i256 { pub hi : i128 , pub lo : u128 , }
    };
}

i256!();