// Generated macro for fuzz_types (macro)
macro_rules! Depcratefuzz_types {
() => {
// Module: crate
// Provides: {"fuzz_types"}
// Dependencies: {}
macro_rules ! fuzz_types { ($ data : ident ; $ ($ type : ty ,) *) => { $ (let _ = <$ type >:: deserialize (& mut &$ data [..]) ;) * } ; }
};
}
