macro_rules! deps {
    () => {
        Endian!();
        FileHeader!();
        SymbolTable!();
        ReadRef!();
    };
}

macro_rules! ElfSymbolTable {
    () => {
        deps!();
        # [doc = " A symbol table in an [`ElfFile`](super::ElfFile)."] # [derive (Debug , Clone , Copy)] pub struct ElfSymbolTable < 'data , 'file , Elf , R = & 'data [u8] > where Elf : FileHeader , R : ReadRef < 'data > , { pub (super) endian : Elf :: Endian , pub (super) symbols : & 'file SymbolTable < 'data , Elf , R > , }
    };
}

ElfSymbolTable!();