macro_rules! deps {
    () => {
        BytesToHexChars!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < 'a > BytesToHexChars < 'a > { fn new (inner : & 'a [u8] , table : & 'static [u8 ; 16]) -> BytesToHexChars < 'a > { BytesToHexChars { inner : inner . iter () , table , next : None , } } }
    };
}

impl_14!();