macro_rules! deps {
    () => {
        Context!();
        LoaderReader!();
    };
}

macro_rules! ObjectContext {
    () => {
        deps!();
        struct ObjectContext < 'a > { ctx : Context < LoaderReader < 'a > > , symbols : SymbolMap < SymbolMapName < 'a > > , }
    };
}

ObjectContext!();