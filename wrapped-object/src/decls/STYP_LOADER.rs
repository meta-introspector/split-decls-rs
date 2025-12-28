macro_rules! STYP_LOADER {
    () => {
        # [doc = " Specifies a loader section. A section of this type contains object file"] # [doc = " information for the system loader to load an XCOFF executable. The information"] # [doc = " includes imported symbols, exported symbols, relocation data, type-check"] # [doc = " information, and shared object names."] pub const STYP_LOADER : u16 = 0x1000 ;
    };
}

STYP_LOADER!();