macro_rules! deps {
    () => {
        ModuleDef!();
        DeclarationLocation!();
    };
}

macro_rules! FileSymbol {
    () => {
        deps!();
        # [doc = " The actual data that is stored in the index. It should be as compact as"] # [doc = " possible."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct FileSymbol { pub name : Symbol , pub def : ModuleDef , pub loc : DeclarationLocation , pub container_name : Option < Symbol > , # [doc = " Whether this symbol is a doc alias for the original symbol."] pub is_alias : bool , pub is_assoc : bool , pub is_import : bool , pub do_not_complete : Complete , }
    };
}

FileSymbol!();