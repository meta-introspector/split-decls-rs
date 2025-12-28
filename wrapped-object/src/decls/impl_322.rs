macro_rules! deps {
    () => {
        SectionIndex!();
        FileHeader!();
        SymbolTable!();
        ReadRef!();
    };
}

macro_rules! impl_322 {
    () => {
        deps!();
        impl < 'data , Elf : FileHeader , R : ReadRef < 'data > > Default for SymbolTable < 'data , Elf , R > { fn default () -> Self { SymbolTable { section : SectionIndex (0) , string_section : SectionIndex (0) , shndx_section : SectionIndex (0) , symbols : & [] , strings : Default :: default () , shndx : & [] , } } }
    };
}

impl_322!();