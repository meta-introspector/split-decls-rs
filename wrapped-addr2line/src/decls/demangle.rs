macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! demangle {
    () => {
        deps!();
        # [doc = " Demangle a symbol name using the demangling scheme for the given language."] # [doc = ""] # [doc = " Returns `None` if demangling failed or is not required."] # [allow (unused_variables)] pub fn demangle (name : & str , language : gimli :: DwLang) -> Option < String > { match language { # [cfg (feature = "rustc-demangle")] gimli :: DW_LANG_Rust => rustc_demangle :: try_demangle (name) . ok () . as_ref () . map (| x | format ! ("{:#}" , x)) , # [cfg (feature = "cpp_demangle")] gimli :: DW_LANG_C_plus_plus | gimli :: DW_LANG_C_plus_plus_03 | gimli :: DW_LANG_C_plus_plus_11 | gimli :: DW_LANG_C_plus_plus_14 | gimli :: DW_LANG_C_plus_plus_17 | gimli :: DW_LANG_C_plus_plus_20 => cpp_demangle :: Symbol :: new (name) . ok () . and_then (| x | x . demangle_with_options (& Default :: default ()) . ok ()) , _ => None , } }
    };
}

demangle!();