macro_rules! deps {
    () => {
        PWSTR!();
    };
}

macro_rules! IMAGEHLP_LINEW64 {
    () => {
        deps!();
        # [repr (C)] # [derive (Clone , Copy)] pub struct IMAGEHLP_LINEW64 { pub SizeOfStruct : u32 , pub Key : * mut core :: ffi :: c_void , pub LineNumber : u32 , pub FileName : PWSTR , pub Address : u64 , }
    };
}

IMAGEHLP_LINEW64!()