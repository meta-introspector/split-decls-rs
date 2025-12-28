macro_rules! deps {
    () => {
        ComPtr!();
        EnumSetupInstances!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        impl EnumSetupInstances { pub unsafe fn from_raw (obj : * mut IEnumSetupInstances) -> EnumSetupInstances { EnumSetupInstances (ComPtr :: from_raw (obj)) } }
    };
}

impl_152!();