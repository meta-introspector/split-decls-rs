macro_rules! deps {
    () => {
        MachOFile!();
        ObjectSymbol!();
        MachHeader!();
        ReadRef!();
        SymbolIndex!();
        Nlist!();
    };
}

macro_rules! MachOSymbol {
    () => {
        deps!();
        # [doc = " A symbol in a [`MachOFile`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSymbol`] trait implementation."] # [derive (Debug , Clone , Copy)] pub struct MachOSymbol < 'data , 'file , Mach , R = & 'data [u8] > where Mach : MachHeader , R : ReadRef < 'data > , { file : & 'file MachOFile < 'data , Mach , R > , index : SymbolIndex , nlist : & 'data Mach :: Nlist , }
    };
}

MachOSymbol!();