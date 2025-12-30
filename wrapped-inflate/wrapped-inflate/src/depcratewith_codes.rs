// Generated macro for with_codes (macro)
macro_rules! Depcratewith_codes {
() => {
// Module: crate
// Provides: {"with_codes"}
// Dependencies: {}
macro_rules ! with_codes (($ clens : expr , $ max_bits : expr => $ code_ty : ty , $ cb : expr) => ({ let mut bl_count = [0 as $ code_ty ; ($ max_bits + 1)] ; for & bits in $ clens . iter () { if bits != 0 { bl_count [bits as usize] += 1 ; } } let mut next_code = [0 as $ code_ty ; ($ max_bits + 1)] ; for bits in 1 ..$ max_bits + 1 { next_code [bits as usize] = (next_code [bits as usize - 1] + bl_count [bits as usize - 1]) << 1 ; } for (i , & bits) in $ clens . iter () . enumerate () { if bits != 0 { let code = next_code [bits as usize] ; next_code [bits as usize] += 1 ; match $ cb (i as $ code_ty , code , bits) { Ok (()) => () , Err (err) => return Err (err) } } } })) ;
};
}
