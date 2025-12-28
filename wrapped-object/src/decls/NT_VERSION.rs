macro_rules! deps {
    () => {
        Note!();
    };
}

macro_rules! NT_VERSION {
    () => {
        deps!();
        # [doc = " Note type for version string."] # [doc = ""] # [doc = " This note may appear in object files."] # [doc = ""] # [doc = " It must be handled as a special case because it has no descriptor, and instead"] # [doc = " uses the note name as the version string."] pub const NT_VERSION : u32 = 1 ;
    };
}

NT_VERSION!();