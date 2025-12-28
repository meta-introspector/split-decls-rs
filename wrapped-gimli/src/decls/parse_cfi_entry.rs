macro_rules! deps {
    () => {
        PartialFrameDescriptionEntry!();
        Result!();
        Error!();
        Reader!();
        CieOffsetEncoding!();
        CommonInformationEntry!();
        UnwindSection!();
        CieOrFde!();
        BaseAddresses!();
        Section!();
    };
}

macro_rules! parse_cfi_entry {
    () => {
        deps!();
        fn parse_cfi_entry < 'bases , Section , R > (bases : & 'bases BaseAddresses , section : & Section , input : & mut R ,) -> Result < Option < CieOrFde < 'bases , Section , R > > > where R : Reader , Section : UnwindSection < R > , { let offset = input . offset_from (section . section ()) ; let (length , format) = input . read_initial_length () ? ; if length . into_u64 () == 0 { return Ok (None) ; } let mut rest = input . split (length) ? ; let cie_offset_base = rest . offset_from (section . section ()) ; let cie_id_or_offset = match Section :: cie_offset_encoding (format) { CieOffsetEncoding :: U32 => rest . read_u32 () . map (u64 :: from) ? , CieOffsetEncoding :: U64 => rest . read_u64 () ? , } ; if Section :: is_cie (format , cie_id_or_offset) { let cie = CommonInformationEntry :: parse_rest (offset , length , format , bases , section , rest) ? ; Ok (Some (CieOrFde :: Cie (cie))) } else { let cie_offset = R :: Offset :: from_u64 (cie_id_or_offset) ? ; let cie_offset = match section . resolve_cie_offset (cie_offset_base , cie_offset) { None => return Err (Error :: OffsetOutOfBounds) , Some (cie_offset) => cie_offset , } ; let fde = PartialFrameDescriptionEntry { offset , length , format , cie_offset : cie_offset . into () , rest , section : section . clone () , bases , } ; Ok (Some (CieOrFde :: Fde (fde))) } }
    };
}

parse_cfi_entry!();