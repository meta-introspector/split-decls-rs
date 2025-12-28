macro_rules! deps {
    () => {
        MachHeader!();
        ReadRef!();
        SymbolTable!();
        StringTable!();
        Endian!();
        SymtabCommand!();
        Result!();
    };
}

macro_rules! impl_561 {
    () => {
        deps!();
        impl < E : Endian > macho :: SymtabCommand < E > { # [doc = " Return the symbol table that this command references."] pub fn symbols < 'data , Mach : MachHeader < Endian = E > , R : ReadRef < 'data > > (& self , endian : E , data : R ,) -> Result < SymbolTable < 'data , Mach , R > > { let symbols = data . read_slice_at (self . symoff . get (endian) . into () , self . nsyms . get (endian) as usize ,) . read_error ("Invalid Mach-O symbol table offset or size") ? ; let str_start : u64 = self . stroff . get (endian) . into () ; let str_end = str_start . checked_add (self . strsize . get (endian) . into ()) . read_error ("Invalid Mach-O string table length") ? ; let strings = StringTable :: new (data , str_start , str_end) ; Ok (SymbolTable :: new (symbols , strings)) } }
    };
}

impl_561!();