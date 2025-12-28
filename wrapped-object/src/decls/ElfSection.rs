macro_rules! deps {
    () => {
        SectionIndex!();
        SectionHeader!();
        FileHeader!();
        ReadRef!();
        ObjectSection!();
        ElfFile!();
    };
}

macro_rules! ElfSection {
    () => {
        deps!();
        # [doc = " A section in an [`ElfFile`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSection`] trait implementation."] # [derive (Debug)] pub struct ElfSection < 'data , 'file , Elf , R = & 'data [u8] > where Elf : FileHeader , R : ReadRef < 'data > , { pub (super) file : & 'file ElfFile < 'data , Elf , R > , pub (super) index : SectionIndex , pub (super) section : & 'data Elf :: SectionHeader , }
    };
}

ElfSection!();