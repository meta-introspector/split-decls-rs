macro_rules! deps {
    () => {
        ImportObjectData!();
    };
}

macro_rules! impl_276 {
    () => {
        deps!();
        impl < 'data > ImportObjectData < 'data > { # [doc = " The public symbol name."] pub fn symbol (& self) -> & 'data [u8] { self . symbol . 0 } # [doc = " The name of the DLL to import the symbol from."] pub fn dll (& self) -> & 'data [u8] { self . dll . 0 } # [doc = " The name exported from the DLL."] # [doc = ""] # [doc = " This is only set if the name is not derived from the symbol name."] pub fn export (& self) -> Option < & 'data [u8] > { self . export . map (| export | export . 0) } }
    };
}

impl_276!()