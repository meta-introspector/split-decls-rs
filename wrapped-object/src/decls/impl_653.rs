macro_rules! deps {
    () => {
        ImageThunkData!();
        ImageNtHeaders!();
        ImageOptionalHeader64!();
        ImageNtHeaders64!();
        ImageOptionalHeader!();
        ImageFileHeader!();
        ImageThunkData64!();
    };
}

macro_rules! impl_653 {
    () => {
        deps!();
        impl ImageNtHeaders for pe :: ImageNtHeaders64 { type ImageOptionalHeader = pe :: ImageOptionalHeader64 ; type ImageThunkData = pe :: ImageThunkData64 ; # [inline] fn is_type_64 (& self) -> bool { true } # [inline] fn is_valid_optional_magic (& self) -> bool { self . optional_header . magic . get (LE) == pe :: IMAGE_NT_OPTIONAL_HDR64_MAGIC } # [inline] fn signature (& self) -> u32 { self . signature . get (LE) } # [inline] fn file_header (& self) -> & pe :: ImageFileHeader { & self . file_header } # [inline] fn optional_header (& self) -> & Self :: ImageOptionalHeader { & self . optional_header } }
    };
}

impl_653!();