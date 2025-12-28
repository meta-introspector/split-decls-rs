macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! BorrowedSymbol {
    () => {
        deps!();
        # [doc = " A `Symbol` which borrows the underlying storage for the mangled name."] pub type BorrowedSymbol < 'a > = Symbol < & 'a [u8] > ;
    };
}

BorrowedSymbol!();