macro_rules! deps {
    () => {
        MetaEnumValue!();
    };
}

macro_rules! __EnumValue {
    () => {
        deps!();
        pub struct __EnumValue < 'a > { pub value : & 'a registry :: MetaEnumValue , }
    };
}

__EnumValue!();