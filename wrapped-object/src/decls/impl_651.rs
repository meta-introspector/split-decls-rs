macro_rules! deps {
    () => {
        ImageThunkData!();
        ImageThunkData32!();
        ImageOptionalHeader!();
        ImageFileHeader!();
        ImageNtHeaders32!();
        ImageNtHeaders!();
        ImageOptionalHeader32!();
    };
}

macro_rules! impl_651 {
    () => {
        deps!();
        impl ImageNtHeaders for pe :: ImageNtHeaders32 { type ImageOptionalHeader = pe :: ImageOptionalHeader32 ; type ImageThunkData = pe :: ImageThunkData32 ; # [inline] fn is_type_64 (& self) -> bool { false } # [inline] fn is_valid_optional_magic (& self) -> bool { self . optional_header . magic . get (LE) == pe :: IMAGE_NT_OPTIONAL_HDR32_MAGIC } # [inline] fn signature (& self) -> u32 { self . signature . get (LE) } # [inline] fn file_header (& self) -> & pe :: ImageFileHeader { & self . file_header } # [inline] fn optional_header (& self) -> & Self :: ImageOptionalHeader { & self . optional_header } }
    };
}

impl_651!()