// Generated macro for BorrowedSymbol (type)
macro_rules! DepcrateBorrowedSymbol {
() => {
// Module: crate
// Provides: {"BorrowedSymbol"}
// Dependencies: {}
# [doc = " A `Symbol` which borrows the underlying storage for the mangled name."] pub type BorrowedSymbol < 'a > = Symbol < & 'a [u8] > ;
};
}
