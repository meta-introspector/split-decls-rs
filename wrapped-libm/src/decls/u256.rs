macro_rules! u256 {
    () => {
        # [doc = " A 256-bit unsigned integer represented as two 128-bit native-endian limbs."] # [allow (non_camel_case_types)] # [derive (Clone , Copy , Debug , PartialEq , PartialOrd , Eq , Ord)] pub struct u256 { pub hi : u128 , pub lo : u128 , }
    };
}

u256!()