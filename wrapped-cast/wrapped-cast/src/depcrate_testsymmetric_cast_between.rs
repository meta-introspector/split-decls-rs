// Generated macro for symmetric_cast_between (macro)
macro_rules! Depcrate_testsymmetric_cast_between {
() => {
// Module: crate::test
// Provides: {"symmetric_cast_between"}
// Dependencies: {}
macro_rules ! symmetric_cast_between { ($ ($ src : ident => $ ($ dst : ident) ,+) ;+;) => { mod symmetric_cast_between { $ (mod $ src { mod and { use quickcheck :: TestResult ; use crate :: From ; $ (quickcheck ! { fn $ dst (src : $ src) -> TestResult { if let Ok (dst) = $ dst :: cast (src) { TestResult :: from_bool ($ src :: cast (dst) . is_ok ()) } else { TestResult :: discard () } } }) + } }) + } } }
};
}
