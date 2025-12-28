macro_rules! deps {
    () => {
        Endian!();
        U32!();
    };
}

macro_rules! DylibTableOfContents {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct DylibTableOfContents < E : Endian > { # [doc = " the defined external symbol (index into the symbol table)"] pub symbol_index : U32 < E > , # [doc = " index into the module table this symbol is defined in"] pub module_index : U32 < E > , }
    };
}

DylibTableOfContents!()