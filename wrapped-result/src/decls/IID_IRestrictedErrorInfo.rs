macro_rules! deps {
    () => {
        GUID!();
    };
}

macro_rules! IID_IRestrictedErrorInfo {
    () => {
        deps!();
        pub const IID_IRestrictedErrorInfo : GUID = GUID :: from_u128 (0x82ba7092_4c88_427d_a7bc_16dd93feb67e) ;
    };
}

IID_IRestrictedErrorInfo!();