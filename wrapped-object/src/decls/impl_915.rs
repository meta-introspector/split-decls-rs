macro_rules! deps {
    () => {
        ObjectMapEntry!();
        ObjectMap!();
        ObjectMapFile!();
    };
}

macro_rules! impl_915 {
    () => {
        deps!();
        impl < 'data > ObjectMapEntry < 'data > { # [doc = " Get the symbol address."] # [inline] pub fn address (& self) -> u64 { self . address } # [doc = " Get the symbol size."] # [doc = ""] # [doc = " This may be 0 if the size is unknown."] # [inline] pub fn size (& self) -> u64 { self . size } # [doc = " Get the symbol name."] # [inline] pub fn name (& self) -> & 'data [u8] { self . name } # [doc = " Get the index of the object file name."] # [inline] pub fn object_index (& self) -> usize { self . object } # [doc = " Get the object file name."] # [inline] pub fn object < 'a > (& self , map : & 'a ObjectMap < 'data >) -> & 'a ObjectMapFile < 'data > { & map . objects [self . object] } }
    };
}

impl_915!();