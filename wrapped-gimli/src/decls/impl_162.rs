macro_rules! deps {
    () => {
        DebugFrame!();
        Vendor!();
        Reader!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl < R : Reader > From < R > for DebugFrame < R > { fn from (section : R) -> Self { DebugFrame { section , address_size : mem :: size_of :: < usize > () as u8 , vendor : Vendor :: Default , } } }
    };
}

impl_162!();