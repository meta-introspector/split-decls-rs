macro_rules! deps {
    () => {
        Symbols!();
    };
}

macro_rules! DynamicSymbols {
    () => {
        deps!();
        # [doc = " A dynamic symbol table."] pub type DynamicSymbols < 'data > = Symbols < 'data , true > ;
    };
}

DynamicSymbols!()