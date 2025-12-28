macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! OwnedSymbol {
    () => {
        deps!();
        # [doc = " A `Symbol` which owns the underlying storage for the mangled name."] pub type OwnedSymbol = Symbol < Vec < u8 > > ;
    };
}

OwnedSymbol!();