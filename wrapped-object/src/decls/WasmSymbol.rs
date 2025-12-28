macro_rules! deps {
    () => {
        WasmFile!();
        ObjectSymbol!();
        SymbolIndex!();
        WasmSymbolInternal!();
    };
}

macro_rules! WasmSymbol {
    () => {
        deps!();
        # [doc = " A symbol in a [`WasmFile`]."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSymbol`] trait implementation."] # [derive (Clone , Copy , Debug)] pub struct WasmSymbol < 'data , 'file > { index : SymbolIndex , symbol : & 'file WasmSymbolInternal < 'data > , }
    };
}

WasmSymbol!()