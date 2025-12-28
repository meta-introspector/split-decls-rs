macro_rules! deps {
    () => {
        U32!();
        ImageFileHeader!();
        ImageOptionalHeader32!();
    };
}

macro_rules! ImageNtHeaders32 {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageNtHeaders32 { pub signature : U32 < LE > , pub file_header : ImageFileHeader , pub optional_header : ImageOptionalHeader32 , }
    };
}

ImageNtHeaders32!()