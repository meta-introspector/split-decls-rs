macro_rules! deps {
    () => {
        ElfSectionIterator!();
        FileHeader!();
        ReadRef!();
        ElfFile!();
    };
}

macro_rules! impl_309 {
    () => {
        deps!();
        impl < 'data , 'file , Elf , R > ElfSectionIterator < 'data , 'file , Elf , R > where Elf : FileHeader , R : ReadRef < 'data > , { pub (super) fn new (file : & 'file ElfFile < 'data , Elf , R >) -> Self { let mut iter = file . sections . iter () . enumerate () ; iter . next () ; ElfSectionIterator { file , iter } } }
    };
}

impl_309!()