macro_rules! deps {
    () => {
        Endian!();
        Bytes!();
        NoteIterator!();
        Error!();
        Result!();
        NoteHeader!();
        Note!();
        FileHeader!();
    };
}

macro_rules! impl_406 {
    () => {
        deps!();
        impl < 'data , Elf > NoteIterator < 'data , Elf > where Elf : FileHeader , { # [doc = " An iterator over the notes in an ELF section or segment."] # [doc = ""] # [doc = " `align` should be from the `p_align` field of the segment,"] # [doc = " or the `sh_addralign` field of the section. Supported values are"] # [doc = " either 4 or 8, but values less than 4 are treated as 4."] # [doc = " This matches the behaviour of binutils."] # [doc = ""] # [doc = " Returns `Err` if `align` is invalid."] pub fn new (endian : Elf :: Endian , align : Elf :: Word , data : & 'data [u8]) -> read :: Result < Self > { let align = match align . into () { 0u64 ..= 4 => 4 , 8 => 8 , _ => return Err (Error ("Invalid ELF note alignment")) , } ; Ok (NoteIterator { endian , align , data : Bytes (data) , }) } # [doc = " Returns the next note."] pub fn next (& mut self) -> read :: Result < Option < Note < 'data , Elf > > > { if self . data . is_empty () { return Ok (None) ; } let result = self . parse () . map (Some) ; if result . is_err () { self . data = Bytes (& []) ; } result } fn parse (& mut self) -> read :: Result < Note < 'data , Elf > > { let header = self . data . read_at :: < Elf :: NoteHeader > (0) . read_error ("ELF note is too short") ? ; let offset = mem :: size_of :: < Elf :: NoteHeader > () ; let namesz = header . n_namesz (self . endian) as usize ; let name = self . data . read_bytes_at (offset , namesz) . read_error ("Invalid ELF note namesz") ? . 0 ; let offset = util :: align (offset + namesz , self . align) ; let descsz = header . n_descsz (self . endian) as usize ; let desc = self . data . read_bytes_at (offset , descsz) . read_error ("Invalid ELF note descsz") ? . 0 ; let offset = util :: align (offset + descsz , self . align) ; if self . data . skip (offset) . is_err () { self . data = Bytes (& []) ; } Ok (Note { header , name , desc }) } }
    };
}

impl_406!();