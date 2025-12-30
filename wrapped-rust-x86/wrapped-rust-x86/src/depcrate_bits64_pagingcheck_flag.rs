// Generated macro for check_flag (macro)
macro_rules! Depcrate_bits64_pagingcheck_flag {
() => {
// Module: crate::bits64::paging
// Provides: {"check_flag"}
// Dependencies: {}
macro_rules ! check_flag { ($ doc : meta , $ fun : ident , $ flag : expr) => { # [$ doc] pub fn $ fun (self) -> bool { self . flags () . contains ($ flag) } } ; }
};
}
