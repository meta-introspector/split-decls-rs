// Generated macro for gen_int (function)
macro_rules! Depcrate_testsgen_int {
() => {
// Module: crate::tests
// Provides: {"gen_int"}
// Dependencies: {}
fn gen_int (i : i64) -> IntCode { use std :: cmp :: Ordering :: { Equal , Greater , Less } ; match i . cmp (& 0) { Greater => IntCode :: Pos (Box :: new (gen_uint (i as u64))) , Less => IntCode :: Neg (Box :: new (gen_uint (i . abs () as u64))) , Equal => IntCode :: Zero , } }
};
}
