macro_rules! FILETIME {
    () => {
        # [repr (C)] # [derive (Clone , Copy , Default)] pub struct FILETIME { pub dwLowDateTime : u32 , pub dwHighDateTime : u32 , }
    };
}

FILETIME!();