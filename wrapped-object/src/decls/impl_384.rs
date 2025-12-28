macro_rules! deps {
    () => {
        ElfFile!();
        ReadRef!();
        ElfComdatIterator!();
        FileHeader!();
    };
}

macro_rules! impl_384 {
    () => {
        deps!();
        impl < 'data , 'file , Elf , R > ElfComdatIterator < 'data , 'file , Elf , R > where Elf : FileHeader , R : ReadRef < 'data > , { pub (super) fn new (file : & 'file ElfFile < 'data , Elf , R > ,) -> ElfComdatIterator < 'data , 'file , Elf , R > { let mut iter = file . sections . iter () . enumerate () ; iter . next () ; ElfComdatIterator { file , iter } } }
    };
}

impl_384!()