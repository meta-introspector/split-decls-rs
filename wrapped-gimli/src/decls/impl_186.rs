macro_rules! deps {
    () => {
        Vendor!();
        DebugFrame!();
        Format!();
        CieOffsetEncoding!();
        Reader!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl < R : Reader > _UnwindSectionPrivate < R > for DebugFrame < R > { fn section (& self) -> & R { & self . section } fn has_zero_terminator () -> bool { false } fn is_cie (format : Format , id : u64) -> bool { match format { Format :: Dwarf32 => id == 0xffff_ffff , Format :: Dwarf64 => id == 0xffff_ffff_ffff_ffff , } } fn cie_offset_encoding (format : Format) -> CieOffsetEncoding { match format { Format :: Dwarf32 => CieOffsetEncoding :: U32 , Format :: Dwarf64 => CieOffsetEncoding :: U64 , } } fn resolve_cie_offset (& self , _ : R :: Offset , offset : R :: Offset) -> Option < R :: Offset > { Some (offset) } fn has_address_and_segment_sizes (version : u8) -> bool { version == 4 } fn address_size (& self) -> u8 { self . address_size } fn vendor (& self) -> Vendor { self . vendor } }
    };
}

impl_186!()