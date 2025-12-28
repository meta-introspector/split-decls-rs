macro_rules! deps {
    () => {
        Reader!();
        EhFrame!();
        Vendor!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        impl < R : Reader > EhFrame < R > { # [doc = " Set the size of a target address in bytes."] # [doc = ""] # [doc = " This defaults to the native word size."] pub fn set_address_size (& mut self , address_size : u8) { self . address_size = address_size } # [doc = " Set the vendor extensions to use."] # [doc = ""] # [doc = " This defaults to `Vendor::Default`."] pub fn set_vendor (& mut self , vendor : Vendor) { self . vendor = vendor ; } }
    };
}

impl_176!();