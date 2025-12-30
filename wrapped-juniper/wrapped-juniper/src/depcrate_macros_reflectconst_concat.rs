// Generated macro for const_concat (macro)
macro_rules! Depcrate_macros_reflectconst_concat {
() => {
// Module: crate::macros::reflect
// Provides: {"const_concat"}
// Dependencies: {}
# [doc = " Concatenates `const` [`str`](prim@str)s in a `const` context."] # [macro_export] macro_rules ! const_concat { ($ ($ s : expr) ,* $ (,) ?) => { { const LEN : :: core :: primitive :: usize = 0 $ (+ $ s . as_bytes () . len ()) *; const CNT : :: core :: primitive :: usize = [$ ($ s) ,*] . len () ; const fn concat (input : [&:: core :: primitive :: str ; CNT]) -> [:: core :: primitive :: u8 ; LEN] { let mut bytes = [0 ; LEN] ; let (mut i , mut byte) = (0 , 0) ; while i < CNT { let mut b = 0 ; while b < input [i] . len () { bytes [byte] = input [i] . as_bytes () [b] ; byte += 1 ; b += 1 ; } i += 1 ; } bytes } const CON : [:: core :: primitive :: u8 ; LEN] = concat ([$ ($ s) ,*]) ; match :: core :: str :: from_utf8 (& CON) { :: core :: result :: Result :: Ok (s) => s , _ => :: core :: unreachable ! () , } } } ; }
};
}
