macro_rules! deps {
    () => {
        Patch!();
    };
}

macro_rules! fix_replacements {
    () => {
        deps!();
        fn fix_replacements (replacements : & mut Map < String , Patch > , dir : & Path) { replacements . remove ("macrotest") ; for replacement in replacements . values_mut () { replacement . path = replacement . path . as_ref () . map (| path | dir . join (path)) ; } }
    };
}

fix_replacements!()