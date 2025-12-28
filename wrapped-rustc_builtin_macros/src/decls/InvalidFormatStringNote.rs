macro_rules! InvalidFormatStringNote {
    () => {
        # [derive (Subdiagnostic)] # [note (builtin_macros_note)] pub (crate) struct InvalidFormatStringNote { pub (crate) note : String , }
    };
}

InvalidFormatStringNote!();