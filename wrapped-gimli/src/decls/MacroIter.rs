macro_rules! deps {
    () => {
        Reader!();
        Format!();
    };
}

macro_rules! MacroIter {
    () => {
        deps!();
        # [doc = " Iterator over the entries in the `.debug_macro` section."] # [derive (Clone , Debug)] pub struct MacroIter < R : Reader > { input : R , format : Format , is_macro : bool , }
    };
}

MacroIter!();