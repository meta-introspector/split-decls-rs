macro_rules! impl_77 {
    () => {
        impl DwLang { # [doc = " Get the default DW_AT_lower_bound for this language."] pub fn default_lower_bound (self) -> Option < usize > { match self { DW_LANG_C89 | DW_LANG_C | DW_LANG_C_plus_plus | DW_LANG_Java | DW_LANG_C99 | DW_LANG_ObjC | DW_LANG_ObjC_plus_plus | DW_LANG_UPC | DW_LANG_D | DW_LANG_Python | DW_LANG_OpenCL | DW_LANG_Go | DW_LANG_Haskell | DW_LANG_C_plus_plus_03 | DW_LANG_C_plus_plus_11 | DW_LANG_OCaml | DW_LANG_Rust | DW_LANG_C11 | DW_LANG_Swift | DW_LANG_Dylan | DW_LANG_C_plus_plus_14 | DW_LANG_RenderScript | DW_LANG_BLISS => Some (0) , DW_LANG_Ada83 | DW_LANG_Cobol74 | DW_LANG_Cobol85 | DW_LANG_Fortran77 | DW_LANG_Fortran90 | DW_LANG_Pascal83 | DW_LANG_Modula2 | DW_LANG_Ada95 | DW_LANG_Fortran95 | DW_LANG_PLI | DW_LANG_Modula3 | DW_LANG_Julia | DW_LANG_Fortran03 | DW_LANG_Fortran08 => Some (1) , _ => None , } } }
    };
}

impl_77!();