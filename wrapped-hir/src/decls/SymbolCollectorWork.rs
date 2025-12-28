macro_rules! SymbolCollectorWork {
    () => {
        # [doc = " Represents an outstanding module that the symbol collector must collect symbols from."] # [derive (Debug)] struct SymbolCollectorWork { module_id : ModuleId , parent : Option < Name > , }
    };
}

SymbolCollectorWork!();