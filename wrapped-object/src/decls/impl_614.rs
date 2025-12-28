macro_rules! deps {
    () => {
        ReadRef!();
        MachOSymbol!();
        SymbolIndex!();
        MachOFile!();
        Nlist!();
        MachHeader!();
    };
}

macro_rules! impl_614 {
    () => {
        deps!();
        impl < 'data , 'file , Mach , R > MachOSymbol < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { pub (super) fn new (file : & 'file MachOFile < 'data , Mach , R > , index : SymbolIndex , nlist : & 'data Mach :: Nlist ,) -> Option < Self > { if nlist . n_type () & macho :: N_STAB != 0 { return None ; } Some (MachOSymbol { file , index , nlist }) } # [doc = " Get the Mach-O file containing this symbol."] pub fn macho_file (& self) -> & 'file MachOFile < 'data , Mach , R > { self . file } # [doc = " Get the raw Mach-O symbol structure."] pub fn macho_symbol (& self) -> & 'data Mach :: Nlist { self . nlist } }
    };
}

impl_614!();