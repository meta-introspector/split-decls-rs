macro_rules! deps {
    () => {
        SymbolSection!();
        SymbolScope!();
        SymbolKind!();
    };
}

macro_rules! WasmSymbolInternal {
    () => {
        deps!();
        # [derive (Clone , Debug)] struct WasmSymbolInternal < 'data > { name : & 'data str , address : u64 , size : u64 , kind : SymbolKind , section : SymbolSection , scope : SymbolScope , weak : bool , }
    };
}

WasmSymbolInternal!();