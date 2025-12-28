macro_rules! deps {
    () => {
        ImageFileHeader!();
        ImageRomOptionalHeader!();
    };
}

macro_rules! ImageRomHeaders {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageRomHeaders { pub file_header : ImageFileHeader , pub optional_header : ImageRomOptionalHeader , }
    };
}

ImageRomHeaders!();