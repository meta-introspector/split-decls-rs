macro_rules! deps {
    () => {
        GUID!();
    };
}

macro_rules! IID_IErrorInfo {
    () => {
        deps!();
        pub const IID_IErrorInfo : GUID = GUID :: from_u128 (0x1cf2b120_547d_101b_8e65_08002b2bd119) ;
    };
}

IID_IErrorInfo!()