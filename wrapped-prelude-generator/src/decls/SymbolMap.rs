macro_rules! SymbolMap {
    () => {
        # [derive (Debug)] pub struct SymbolMap { pub map : HashMap < String , ResolvedDependency > , }
    };
}

SymbolMap!();