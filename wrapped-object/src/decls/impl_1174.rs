macro_rules! deps {
    () => {
        Symbol!();
        AttributeTag!();
        Section!();
        File!();
    };
}

macro_rules! impl_1174 {
    () => {
        deps!();
        impl AttributeTag { # [doc = " Return the corresponding `elf::Tag_*` value for this tag."] pub fn tag (& self) -> u8 { match self { AttributeTag :: File => elf :: Tag_File , AttributeTag :: Section (_) => elf :: Tag_Section , AttributeTag :: Symbol (_) => elf :: Tag_Symbol , } } }
    };
}

impl_1174!();