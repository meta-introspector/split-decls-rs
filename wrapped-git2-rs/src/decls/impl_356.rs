macro_rules! deps {
    () => {
        Binding!();
        DiffBinaryFile!();
        DiffBinaryKind!();
    };
}

macro_rules! impl_356 {
    () => {
        deps!();
        impl < 'a > DiffBinaryFile < 'a > { # [doc = " The type of binary data for this file"] pub fn kind (& self) -> DiffBinaryKind { unsafe { Binding :: from_raw ((* self . raw) . kind) } } # [doc = " The binary data, deflated"] pub fn data (& self) -> & [u8] { unsafe { slice :: from_raw_parts ((* self . raw) . data as * const u8 , (* self . raw) . datalen as usize) } } # [doc = " The length of the binary data after inflation"] pub fn inflated_len (& self) -> usize { unsafe { (* self . raw) . inflatedlen as usize } } }
    };
}

impl_356!()