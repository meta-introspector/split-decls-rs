macro_rules! MacroSubNs {
    () => {
        # [doc = " See `sub_namespace_match()`."] # [derive (Clone , Copy , PartialEq , Eq)] pub enum MacroSubNs { # [doc = " Function-like macros, suffixed with `!`."] Bang , # [doc = " Macros inside attributes, i.e. attribute macros and derive macros."] Attr , }
    };
}

MacroSubNs!();