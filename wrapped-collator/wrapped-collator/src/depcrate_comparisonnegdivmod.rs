// Generated macro for negdivmod (macro)
macro_rules! Depcrate_comparisonnegdivmod {
() => {
// Module: crate::comparison
// Provides: {"negdivmod"}
// Dependencies: {}
macro_rules ! negdivmod { ($ n : ident , $ d : ident , $ m : ident) => { $ m = $ n % $ d ; $ n /= $ d ; if $ m < 0 { $ n -= 1 ; $ m += $ d ; } } ; }
};
}
