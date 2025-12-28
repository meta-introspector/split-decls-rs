macro_rules! deps {
    () => {
        Reader!();
        Vendor!();
        EhFrame!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        impl < R : Reader > From < R > for EhFrame < R > { fn from (section : R) -> Self { EhFrame { section , address_size : mem :: size_of :: < usize > () as u8 , vendor : Vendor :: Default , } } }
    };
}

impl_179!();