// Generated macro for round_up_nonzero (macro)
macro_rules! Depcrate_slowround_up_nonzero {
() => {
// Module: crate::slow
// Provides: {"round_up_nonzero"}
// Dependencies: {}
# [doc = " Check and round-up the fraction if any non-zero digits exist."] macro_rules ! round_up_nonzero { ($ format : ident , $ iter : expr , $ result : ident , $ count : ident) => { { for & digit in $ iter { if digit != b'0' { round_up_truncated ! ($ format , $ result , $ count) ; return ($ result , $ count) ; } } } } ; }
};
}
