macro_rules! deps {
    () => {
        Rel32!();
        ReadRef!();
        Error!();
        Rel!();
        FileHeader32!();
        SectionHeader32!();
        Result!();
        SectionHeader!();
    };
}

macro_rules! impl_802 {
    () => {
        deps!();
        impl SectionHeader for xcoff :: SectionHeader32 { type Word = u32 ; type HalfWord = u16 ; type Xcoff = xcoff :: FileHeader32 ; type Rel = xcoff :: Rel32 ; fn s_name (& self) -> & [u8 ; 8] { & self . s_name } fn s_paddr (& self) -> Self :: Word { self . s_paddr . get (BE) } fn s_vaddr (& self) -> Self :: Word { self . s_vaddr . get (BE) } fn s_size (& self) -> Self :: Word { self . s_size . get (BE) } fn s_scnptr (& self) -> Self :: Word { self . s_scnptr . get (BE) } fn s_relptr (& self) -> Self :: Word { self . s_relptr . get (BE) } fn s_lnnoptr (& self) -> Self :: Word { self . s_lnnoptr . get (BE) } fn s_nreloc (& self) -> Self :: HalfWord { self . s_nreloc . get (BE) } fn s_nlnno (& self) -> Self :: HalfWord { self . s_nlnno . get (BE) } fn s_flags (& self) -> u32 { self . s_flags . get (BE) } # [doc = " Read the relocations in a XCOFF32 file."] # [doc = ""] # [doc = " `data` must be the entire file data."] fn relocations < 'data , R : ReadRef < 'data > > (& self , data : R) -> read :: Result < & 'data [Self :: Rel] > { let reloc_num = self . s_nreloc () as usize ; if reloc_num == 65535 { return Err (Error ("Overflow section is not supported yet.")) ; } data . read_slice_at (self . s_relptr () . into () , reloc_num) . read_error ("Invalid XCOFF relocation offset or number") } }
    };
}

impl_802!();