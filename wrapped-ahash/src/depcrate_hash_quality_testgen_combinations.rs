// Generated macro for gen_combinations (function)
macro_rules! Depcrate_hash_quality_testgen_combinations {
() => {
// Module: crate::hash_quality_test
// Provides: {"gen_combinations"}
// Dependencies: {}
fn gen_combinations (options : & [u32 ; 11] , depth : u32 , so_far : Vec < u32 > , combinations : & mut Vec < Vec < u32 > >) { if depth == 0 { return ; } for option in options { let mut next = so_far . clone () ; next . push (* option) ; combinations . push (next . clone ()) ; gen_combinations (options , depth - 1 , next , combinations) ; } }
};
}
