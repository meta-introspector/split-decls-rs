macro_rules! deps {
    () => {
        Error!();
        Result!();
        LoadCommandData!();
        LoadCommand!();
        Endian!();
        Bytes!();
        LoadCommandIterator!();
    };
}

macro_rules! impl_556 {
    () => {
        deps!();
        impl < 'data , E : Endian > LoadCommandIterator < 'data , E > { pub (super) fn new (endian : E , data : & 'data [u8] , ncmds : u32) -> Self { LoadCommandIterator { endian , data : Bytes (data) , ncmds , } } # [doc = " Return the next load command."] pub fn next (& mut self) -> Result < Option < LoadCommandData < 'data , E > > > { if self . ncmds == 0 { return Ok (None) ; } let result = self . parse () . map (Some) ; if result . is_err () { self . ncmds = 0 ; } else { self . ncmds -= 1 ; } result } fn parse (& mut self) -> Result < LoadCommandData < 'data , E > > { let header = self . data . read_at :: < macho :: LoadCommand < E > > (0) . read_error ("Invalid Mach-O load command header") ? ; let cmd = header . cmd . get (self . endian) ; let cmdsize = header . cmdsize . get (self . endian) as usize ; if cmdsize < mem :: size_of :: < macho :: LoadCommand < E > > () { return Err (Error ("Invalid Mach-O load command size")) ; } let data = self . data . read_bytes (cmdsize) . read_error ("Invalid Mach-O load command size") ? ; Ok (LoadCommandData { cmd , data , marker : Default :: default () , }) } }
    };
}

impl_556!();