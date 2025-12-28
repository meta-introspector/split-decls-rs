macro_rules! deps {
    () => {
        RelocationIterator!();
    };
}

macro_rules! impl_715 {
    () => {
        deps!();
        impl < 'data > RelocationIterator < 'data > { # [doc = " Return the virtual address of the page that this block of relocations applies to."] pub fn virtual_address (& self) -> u32 { self . virtual_address } # [doc = " Return the size in bytes of this block of relocations."] pub fn size (& self) -> u32 { self . size } }
    };
}

impl_715!()