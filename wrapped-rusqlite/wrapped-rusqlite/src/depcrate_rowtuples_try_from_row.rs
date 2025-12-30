// Generated macro for tuples_try_from_row (macro)
macro_rules! Depcrate_rowtuples_try_from_row {
() => {
// Module: crate::row
// Provides: {"tuples_try_from_row"}
// Dependencies: {}
macro_rules ! tuples_try_from_row { () => { tuple_try_from_row ! () ; } ; ($ first : ident $ (, $ remaining : ident) *) => { tuple_try_from_row ! ($ first $ (, $ remaining) *) ; tuples_try_from_row ! ($ ($ remaining) ,*) ; } ; }
};
}
