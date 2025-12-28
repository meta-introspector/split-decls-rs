macro_rules! deps {
    () => {
        SectionHeader!();
        U32Bytes!();
        ObjectComdat!();
        Endian!();
        FileHeader!();
        ReadRef!();
        ElfFile!();
    };
}

macro_rules! ElfComdat {
    () => {
        deps!();
        # [doc = " A COMDAT section group in an [`ElfFile`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectComdat`] trait implementation."] # [derive (Debug)] pub struct ElfComdat < 'data , 'file , Elf , R = & 'data [u8] > where Elf : FileHeader , R : ReadRef < 'data > , { file : & 'file ElfFile < 'data , Elf , R > , section : & 'data Elf :: SectionHeader , sections : & 'data [U32Bytes < Elf :: Endian >] , }
    };
}

ElfComdat!();