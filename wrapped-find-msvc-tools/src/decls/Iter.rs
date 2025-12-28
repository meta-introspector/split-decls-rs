macro_rules! deps {
    () => {
        RegistryKey!();
        DWORD!();
    };
}

macro_rules! Iter {
    () => {
        deps!();
        pub struct Iter < 'a > { idx : RangeFrom < DWORD > , key : & 'a RegistryKey , }
    };
}

Iter!()