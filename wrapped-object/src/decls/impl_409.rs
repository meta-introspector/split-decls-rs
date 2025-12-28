macro_rules! deps {
    () => {
        Bytes!();
        Endian!();
        NoteHeader!();
        Note!();
        GnuPropertyIterator!();
        FileHeader!();
    };
}

macro_rules! impl_409 {
    () => {
        deps!();
        impl < 'data , Elf : FileHeader > Note < 'data , Elf > { # [doc = " Return the `n_type` field of the `NoteHeader`."] # [doc = ""] # [doc = " The meaning of this field is determined by `name`."] pub fn n_type (& self , endian : Elf :: Endian) -> u32 { self . header . n_type (endian) } # [doc = " Return the `n_namesz` field of the `NoteHeader`."] pub fn n_namesz (& self , endian : Elf :: Endian) -> u32 { self . header . n_namesz (endian) } # [doc = " Return the `n_descsz` field of the `NoteHeader`."] pub fn n_descsz (& self , endian : Elf :: Endian) -> u32 { self . header . n_descsz (endian) } # [doc = " Return the bytes for the name field following the `NoteHeader`."] # [doc = ""] # [doc = " This field is usually a string including one or more trailing null bytes"] # [doc = " (but it is not required to be)."] # [doc = ""] # [doc = " The length of this field is given by `n_namesz`."] pub fn name_bytes (& self) -> & 'data [u8] { self . name } # [doc = " Return the bytes for the name field following the `NoteHeader`,"] # [doc = " excluding all trailing null bytes."] pub fn name (& self) -> & 'data [u8] { let mut name = self . name ; while let [rest @ .. , 0] = name { name = rest ; } name } # [doc = " Return the bytes for the desc field following the `NoteHeader`."] # [doc = ""] # [doc = " The length of this field is given by `n_descsz`. The meaning"] # [doc = " of this field is determined by `name` and `n_type`."] pub fn desc (& self) -> & 'data [u8] { self . desc } # [doc = " Return an iterator for properties if this note's type is [`elf::NT_GNU_PROPERTY_TYPE_0`]."] pub fn gnu_properties (& self , endian : Elf :: Endian ,) -> Option < GnuPropertyIterator < 'data , Elf :: Endian > > { if self . name () != elf :: ELF_NOTE_GNU || self . n_type (endian) != elf :: NT_GNU_PROPERTY_TYPE_0 { return None ; } let align = if Elf :: is_type_64_sized () { 8 } else { 4 } ; Some (GnuPropertyIterator { endian , align , data : Bytes (self . desc) , }) } }
    };
}

impl_409!();