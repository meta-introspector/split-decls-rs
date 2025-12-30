// Generated macro for round_up_truncated (macro)
macro_rules! Depcrate_slowround_up_truncated {
() => {
// Module: crate::slow
// Provides: {"round_up_truncated"}
// Dependencies: {}
# [doc = " Round-up a truncated value."] macro_rules ! round_up_truncated { ($ format : ident , $ result : ident , $ count : ident) => { { add_temporary ! (@ mul $ result , 10 , 1) ; $ count += 1 ; } } ; }
};
}
