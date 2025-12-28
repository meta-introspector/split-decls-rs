macro_rules! deps {
    () => {
        WasmFile!();
        WasmSymbolInternal!();
    };
}

macro_rules! WasmSymbolIterator {
    () => {
        deps!();
        # [doc = " An iterator for the symbols in a [`WasmFile`]."] # [derive (Debug)] pub struct WasmSymbolIterator < 'data , 'file > { symbols : core :: iter :: Enumerate < slice :: Iter < 'file , WasmSymbolInternal < 'data > > > , }
    };
}

WasmSymbolIterator!()