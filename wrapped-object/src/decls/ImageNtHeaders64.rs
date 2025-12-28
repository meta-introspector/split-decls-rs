macro_rules! deps {
    () => {
        U32!();
        ImageFileHeader!();
        ImageOptionalHeader64!();
    };
}

macro_rules! ImageNtHeaders64 {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageNtHeaders64 { pub signature : U32 < LE > , pub file_header : ImageFileHeader , pub optional_header : ImageOptionalHeader64 , }
    };
}

ImageNtHeaders64!()