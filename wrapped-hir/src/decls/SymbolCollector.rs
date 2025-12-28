macro_rules! deps {
    () => {
        SymbolCollectorWork!();
        FileSymbol!();
    };
}

macro_rules! SymbolCollector {
    () => {
        deps!();
        pub struct SymbolCollector < 'a > { db : & 'a dyn HirDatabase , symbols : FxIndexSet < FileSymbol > , work : Vec < SymbolCollectorWork > , current_container_name : Option < Symbol > , collect_pub_only : bool , }
    };
}

SymbolCollector!();