macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! DynamicSymbol {
    () => {
        deps!();
        # [doc = " A dynamic symbol."] pub type DynamicSymbol < 'data > = Symbol < 'data , true > ;
    };
}

DynamicSymbol!();