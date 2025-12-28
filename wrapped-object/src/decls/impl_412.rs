macro_rules! deps {
    () => {
        NoteHeader64!();
        Endian!();
        NoteHeader!();
    };
}

macro_rules! impl_412 {
    () => {
        deps!();
        impl < Endian : endian :: Endian > NoteHeader for elf :: NoteHeader64 < Endian > { type Endian = Endian ; # [inline] fn n_namesz (& self , endian : Self :: Endian) -> u32 { self . n_namesz . get (endian) } # [inline] fn n_descsz (& self , endian : Self :: Endian) -> u32 { self . n_descsz . get (endian) } # [inline] fn n_type (& self , endian : Self :: Endian) -> u32 { self . n_type . get (endian) } }
    };
}

impl_412!()