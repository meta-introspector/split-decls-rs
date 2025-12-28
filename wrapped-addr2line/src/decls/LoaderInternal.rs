macro_rules! deps {
    () => {
        ObjectContext!();
        Context!();
        LoaderReader!();
    };
}

macro_rules! LoaderInternal {
    () => {
        deps!();
        struct LoaderInternal < 'a > { ctx : Context < LoaderReader < 'a > > , object : object :: File < 'a > , relative_address_base : u64 , symbols : SymbolMap < SymbolMapName < 'a > > , dwarf_package : Option < gimli :: DwarfPackage < LoaderReader < 'a > > > , object_map : object :: ObjectMap < 'a > , objects : Vec < OnceCell < Option < ObjectContext < 'a > > > > , }
    };
}

LoaderInternal!();