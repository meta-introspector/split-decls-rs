macro_rules! deps {
    () => {
        IndexStr!();
    };
}

macro_rules! impl_328 {
    () => {
        deps!();
        impl < 'a , 'b > PartialEq < & 'a [u8] > for IndexStr < 'b > { fn eq (& self , rhs : & & [u8]) -> bool { self . string == * rhs } }
    };
}

impl_328!();