macro_rules! deps {
    () => {
        ReadRef!();
        Pod!();
        Result!();
        StringTable!();
    };
}

macro_rules! FileAux {
    () => {
        deps!();
        # [doc = " A trait for generic access to [`xcoff::FileAux32`] and [`xcoff::FileAux64`]."] # [allow (missing_docs)] pub trait FileAux : Debug + Pod { fn x_fname (& self) -> & [u8 ; 8] ; fn x_ftype (& self) -> u8 ; fn x_auxtype (& self) -> Option < u8 > ; fn name_offset (& self) -> Option < u32 > { let x_fname = self . x_fname () ; if x_fname [0] == 0 { Some (u32 :: from_be_bytes (x_fname [4 .. 8] . try_into () . unwrap ())) } else { None } } # [doc = " Parse the x_fname field, which may be an inline string or a string table offset."] fn fname < 'data , R : ReadRef < 'data > > (& 'data self , strings : StringTable < 'data , R > ,) -> Result < & 'data [u8] > { if let Some (offset) = self . name_offset () { strings . get (offset) . read_error ("Invalid XCOFF symbol name offset") } else { let x_fname = self . x_fname () ; Ok (match memchr :: memchr (b'\0' , x_fname) { Some (end) => & x_fname [.. end] , None => x_fname , }) } } }
    };
}

FileAux!()