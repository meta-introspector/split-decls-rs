macro_rules! deps {
    () => {
        WasmFile!();
        WasmSymbolInternal!();
    };
}

macro_rules! WasmSymbolTable {
    () => {
        deps!();
        # [doc = " A symbol table in a [`WasmFile`]."] # [derive (Debug)] pub struct WasmSymbolTable < 'data , 'file > { symbols : & 'file [WasmSymbolInternal < 'data >] , }
    };
}

WasmSymbolTable!();