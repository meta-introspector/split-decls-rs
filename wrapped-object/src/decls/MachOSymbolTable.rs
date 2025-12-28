macro_rules! deps {
    () => {
        MachOFile!();
        ReadRef!();
        MachHeader!();
    };
}

macro_rules! MachOSymbolTable {
    () => {
        deps!();
        # [doc = " A symbol table in a [`MachOFile`]."] # [derive (Debug , Clone , Copy)] pub struct MachOSymbolTable < 'data , 'file , Mach , R = & 'data [u8] > where Mach : MachHeader , R : ReadRef < 'data > , { pub (super) file : & 'file MachOFile < 'data , Mach , R > , }
    };
}

MachOSymbolTable!();