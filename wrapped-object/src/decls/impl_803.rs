macro_rules! deps {
    () => {
        SectionHeader!();
        Result!();
        FileHeader64!();
        ReadRef!();
        Rel64!();
        SectionHeader64!();
        Rel!();
    };
}

macro_rules! impl_803 {
    () => {
        deps!();
        impl SectionHeader for xcoff :: SectionHeader64 { type Word = u64 ; type HalfWord = u32 ; type Xcoff = xcoff :: FileHeader64 ; type Rel = xcoff :: Rel64 ; fn s_name (& self) -> & [u8 ; 8] { & self . s_name } fn s_paddr (& self) -> Self :: Word { self . s_paddr . get (BE) } fn s_vaddr (& self) -> Self :: Word { self . s_vaddr . get (BE) } fn s_size (& self) -> Self :: Word { self . s_size . get (BE) } fn s_scnptr (& self) -> Self :: Word { self . s_scnptr . get (BE) } fn s_relptr (& self) -> Self :: Word { self . s_relptr . get (BE) } fn s_lnnoptr (& self) -> Self :: Word { self . s_lnnoptr . get (BE) } fn s_nreloc (& self) -> Self :: HalfWord { self . s_nreloc . get (BE) } fn s_nlnno (& self) -> Self :: HalfWord { self . s_nlnno . get (BE) } fn s_flags (& self) -> u32 { self . s_flags . get (BE) } # [doc = " Read the relocations in a XCOFF64 file."] # [doc = ""] # [doc = " `data` must be the entire file data."] fn relocations < 'data , R : ReadRef < 'data > > (& self , data : R) -> read :: Result < & 'data [Self :: Rel] > { data . read_slice_at (self . s_relptr () , self . s_nreloc () as usize) . read_error ("Invalid XCOFF relocation offset or number") } }
    };
}

impl_803!();