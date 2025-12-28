macro_rules! deps {
    () => {
        DebugStrOffsets!();
        Reader!();
        DebugStrOffsetsIndex!();
        Format!();
        DebugStrOffsetsBase!();
        Result!();
        DebugStrOffset!();
    };
}

macro_rules! impl_589 {
    () => {
        deps!();
        impl < R : Reader > DebugStrOffsets < R > { # [doc = " Returns the `.debug_str` offset at the given `base` and `index`."] # [doc = ""] # [doc = " A set of entries in the `.debug_str_offsets` section consists of a header"] # [doc = " followed by a series of string table offsets."] # [doc = ""] # [doc = " The `base` must be the `DW_AT_str_offsets_base` value from the compilation unit DIE."] # [doc = " This is an offset that points to the first entry following the header."] # [doc = ""] # [doc = " The `index` is the value of a `DW_FORM_strx` attribute."] # [doc = ""] # [doc = " The `format` must be the DWARF format of the compilation unit. This format must"] # [doc = " match the header. However, note that we do not parse the header to validate this,"] # [doc = " since locating the header is unreliable, and the GNU extensions do not emit it."] pub fn get_str_offset (& self , format : Format , base : DebugStrOffsetsBase < R :: Offset > , index : DebugStrOffsetsIndex < R :: Offset > ,) -> Result < DebugStrOffset < R :: Offset > > { let input = & mut self . section . clone () ; input . skip (base . 0) ? ; input . skip (R :: Offset :: from_u64 (index . 0 . into_u64 () * u64 :: from (format . word_size ()) ,) ?) ? ; input . read_offset (format) . map (DebugStrOffset) } }
    };
}

impl_589!();