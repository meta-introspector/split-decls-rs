macro_rules! deps {
    () => {
        EhFrame!();
        CieOffsetEncoding!();
        Vendor!();
        Reader!();
        Format!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl < R : Reader > _UnwindSectionPrivate < R > for EhFrame < R > { fn section (& self) -> & R { & self . section } fn has_zero_terminator () -> bool { true } fn is_cie (_ : Format , id : u64) -> bool { id == 0 } fn cie_offset_encoding (_format : Format) -> CieOffsetEncoding { CieOffsetEncoding :: U32 } fn resolve_cie_offset (& self , base : R :: Offset , offset : R :: Offset) -> Option < R :: Offset > { base . checked_sub (offset) } fn has_address_and_segment_sizes (_version : u8) -> bool { false } fn address_size (& self) -> u8 { self . address_size } fn vendor (& self) -> Vendor { self . vendor } }
    };
}

impl_188!();