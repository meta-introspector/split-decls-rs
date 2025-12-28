macro_rules! deps {
    () => {
        Rel!();
        Endian!();
        Crel!();
        SymbolIndex!();
        Rela!();
    };
}

macro_rules! impl_374 {
    () => {
        deps!();
        impl Crel { # [doc = " Get the symbol index referenced by the relocation."] # [doc = ""] # [doc = " Returns `None` for the null symbol index."] pub fn symbol (& self) -> Option < SymbolIndex > { if self . r_sym == 0 { None } else { Some (SymbolIndex (self . r_sym as usize)) } } # [doc = " Build Crel type from Rel."] pub fn from_rel < R : Rel > (r : & R , endian : R :: Endian) -> Crel { Crel { r_offset : r . r_offset (endian) . into () , r_sym : r . r_sym (endian) , r_type : r . r_type (endian) , r_addend : 0 , } } # [doc = " Build Crel type from Rela."] pub fn from_rela < R : Rela > (r : & R , endian : R :: Endian , is_mips64el : bool) -> Crel { Crel { r_offset : r . r_offset (endian) . into () , r_sym : r . r_sym (endian , is_mips64el) , r_type : r . r_type (endian , is_mips64el) , r_addend : r . r_addend (endian) . into () , } } }
    };
}

impl_374!();